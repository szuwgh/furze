use crate::encoder::Encoder;
use crate::error::FstError;
use crate::error::FstResult;
use crate::state::State;
use crate::state::{
    ARCS_AS_FIXED_ARRAY, BIT_FINAL_STATE, BIT_LAST_STATE, BIT_STATE_HAS_FINAL_OUTPUT,
    BIT_STATE_HAS_OUPPUT, BIT_STOP_NODE, BIT_TAGET_NEXT,
};
use byteorder::BigEndian;
use byteorder::ReadBytesExt;
use std::io::{Error as IOError, Result};
use varintrs::{Binary, ReadBytesVarExt};
const DROP_MSB: u8 = 0b0111_1111;
const MSB: u8 = 0b1000_0000;

#[macro_export]
macro_rules! copy {
    ($des:expr, $src:expr) => {
        copy_slice($des, $src)
    };
}

fn copy_slice<T: Copy>(des: &mut [T], src: &[T]) -> usize {
    let l = if des.len() < src.len() {
        des.len()
    } else {
        src.len()
    };
    unsafe {
        std::ptr::copy_nonoverlapping(src.as_ptr(), des.as_mut_ptr(), l);
    }
    l
}

struct ReverseReader<'a> {
    i: i32,
    data: &'a [u8],
}

impl<'a> ReverseReader<'a> {
    fn new(data: &'a [u8]) -> ReverseReader {
        Self {
            i: (data.len() - 1) as i32,
            data: data,
        }
    }

    fn reset(&mut self) {
        self.i = (self.data.len() - 1) as i32;
    }

    fn set_position(&mut self, postion: i32) {
        self.i = postion
    }

    fn get_bytes(&self, start: usize, end: usize) -> &'a [u8] {
        &self.data[start..end]
    }

    fn read_byte(&mut self) -> Result<u8> {
        if self.i < 0 {
            return Err(IOError::from(std::io::ErrorKind::UnexpectedEof));
        }
        let b = self.data[self.i as usize];
        self.i -= 1;
        Ok(b)
    }

    fn get_position(&self) -> i32 {
        self.i as i32
    }
}

use std::io::Read;
impl<'a> Read for ReverseReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.i < 0 {
            return Err(IOError::from(std::io::ErrorKind::UnexpectedEof));
        }
        for x in buf.iter_mut() {
            *x = self.read_byte()?;
        }
        Ok(buf.len())
    }
}

pub(crate) struct Decoder<'a> {
    reader: ReverseReader<'a>,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Decoder {
        Self {
            reader: ReverseReader::new(data),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.reader.reset()
    }

    fn near_next(&mut self, key: &[u8], state: &mut State) -> FstResult<u64> {
        let mut frist_k = 0;
        if key.len() > 0 {
            frist_k = key[0];
        }
        let mut out: u64 = 0;
        let mut greater: bool = false;
        loop {
            let frist_res = self.near_target_state(frist_k, state);
            let position = self.reader.get_position();
            match frist_res {
                Err(e) => match e {
                    FstError::Greater => {
                        greater = true;
                    }
                    _ => {
                        return Err(FstError::NotFound);
                    }
                },
                Ok(()) => {}
            }
            out += state.out;
            if greater {
                loop {
                    if state.is_final {
                        break;
                    }
                    self.read_first_state(state)?;
                    out += state.out;
                }
                out += state.final_out;
            } else {
                let res = self.near_next(&key[1..], state);
                match res {
                    Ok(_out) => {
                        out += _out;
                    }
                    Err(e) => {
                        if !state.is_last {
                            self.reader.set_position(position);
                            state.reset();
                            continue;
                        }
                        return Err(FstError::NotFound);
                    }
                }
            }
            break;
        }

        Ok(out)
    }

    fn near_target_state(&mut self, _in: u8, state: &mut State) -> FstResult<()> {
        self.read_first_state(state)?;
        loop {
            if state._in == _in {
                return Ok(());
            } else if state._in > _in {
                return Err(FstError::Greater);
            } else if state.is_last {
                return Err(FstError::NotFound);
            } else {
                self.read_next_state(state)?;
            }
        }
    }

    pub(crate) fn find(&mut self, key: &[u8]) -> FstResult<u64> {
        let mut state = State::new(0, 0);
        let mut out: u64 = 0;
        for _k in key.iter() {
            self.find_target_state(*_k, &mut state)?;
            out += state.out;
        }
        if !state.is_final {
            return Err(FstError::NotFound);
        }
        out += state.final_out;
        Ok(out)
    }

    pub(crate) fn near(&mut self, key: &[u8]) -> FstResult<u64> {
        let mut state = State::new(0, 0);
        self.near_next(key, &mut state)
    }

    fn find_target_state(&mut self, _in: u8, state: &mut State) -> FstResult<()> {
        self.read_first_state(state)?;
        if state.flag(ARCS_AS_FIXED_ARRAY) {
            // do binary search
            let num_states = state.final_out;
            let mut low = 0;
            let mut high = num_states - 1;
            let start = self.reader.get_position();
            while low <= high {
                let mid = (low + high) >> 1; // (low + high)/2
                self.reader.set_position(start - (mid * 5) as i32);
                let mid_label = self.reader.read_u8()? as i32;
                let cmp = mid_label - _in as i32;
                if cmp < 0 {
                    low = mid + 1;
                } else if cmp > 0 {
                    high = mid - 1;
                } else {
                    let position = self.reader.read_u32::<BigEndian>()? as i32;
                    self.reader.set_position(position);
                    self.read_next_state(state)?;
                    return Ok(());
                }
            }
            return Err(FstError::NotFound);
        }
        loop {
            if state._in == _in {
                return Ok(());
            } else if state.is_last {
                return Err(FstError::NotFound);
            } else {
                self.read_next_state(state)?;
            }
        }
    }

    fn read_first_state(&mut self, state: &mut State) -> FstResult<()> {
        if state.target > 0 {
            self.reader.set_position(state.target as i32);
        }
        self.read_next_state(state)
    }

    fn read_next_state(&mut self, state: &mut State) -> FstResult<()> {
        state.reset();
        state.flag = self.reader.read_u8()?;
        state._in = self.reader.read_u8()?;
        if state.flag(BIT_STATE_HAS_FINAL_OUTPUT) || state.flag(ARCS_AS_FIXED_ARRAY) {
            let (v, _) = self.reader.read_vu64::<Binary>();
            state.final_out = v;
        }

        if state.flag(BIT_STATE_HAS_OUPPUT) {
            let (v, _) = self.reader.read_vu64::<Binary>();
            state.out = v;
        }
        if state.flag(BIT_STOP_NODE) {
            state.is_stop = true;
        } else {
            if !state.flag(BIT_TAGET_NEXT) && !state.flag(ARCS_AS_FIXED_ARRAY) {
                let (v, _) = self.reader.read_vu64::<Binary>();
                state.target = v;
            }
        }
        if state.flag(BIT_LAST_STATE) {
            state.is_last = true;
        }
        if state.flag(BIT_FINAL_STATE) {
            state.is_final = true;
        }

        Ok(())
    }

    // fn read_byte(&mut self) -> FstResult<u8> {
    //     self.reader.read_byte()
    // }

    // fn read_v_u64(&mut self) -> FstResult<(u64, usize)> {
    //     let mut result: u64 = 0;
    //     let mut shift = 0;
    //     let mut success = false;
    //     loop {
    //         let b = self.read_byte()?;
    //         let msb_dropped = b & DROP_MSB;
    //         result |= (msb_dropped as u64) << shift;
    //         shift += 7;
    //         if b & MSB == 0 || shift > (9 * 7) {
    //             success = b & MSB == 0;
    //             break;
    //         }
    //     }
    //     if success {
    //         Ok((result, shift / 7 as usize))
    //     } else {
    //         Err(FstError::Fail)
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {}
}
