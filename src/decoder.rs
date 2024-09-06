
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
const FINAL_END_NODE: u64 = 2 << 63;
pub(crate) const END_LABEL: u8 = 255;

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

struct ReverseReader<T: AsRef<[u8]>> {
    i: usize,
    data: T,
}

impl<T: AsRef<[u8]>> ReverseReader<T> {
    fn new(data: T) -> ReverseReader<T> {
        Self {
            i: (data.as_ref().len() - 1),
            data: data,
        }
    }

    fn reset(&mut self) {
        self.i = self.data.as_ref().len() - 1;
    }

    fn set_position(&mut self, postion: usize) {
        self.i = postion
    }

    fn skip_bytes(&mut self, skip: usize) {
        self.i = self.i - skip;
    }

    // fn get_bytes(&self, start: usize, end: usize) -> &[u8] {
    //     &self.data.as_ref()[start..end]
    // }

    fn read_byte(&mut self) -> Result<u8> {
        // if self.i == 0 {
        //     return Err(IOError::from(std::io::ErrorKind::UnexpectedEof));
        // }
        let b = self.data.as_ref()[self.i];
        self.i -= 1;
        Ok(b)
    }

    fn get_position(&self) -> usize {
        self.i
    }
}

use std::io::Read;

impl<T: AsRef<[u8]>> Read for ReverseReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.i == 0 {
            return Err(IOError::from(std::io::ErrorKind::UnexpectedEof));
        }
        for x in buf.iter_mut() {
            *x = self.read_byte()?;
        }
        Ok(buf.len())
    }
}

pub(crate) struct Decoder<T: AsRef<[u8]>> {
    reader: ReverseReader<T>,
}

impl<T: AsRef<[u8]>> Decoder<T> {
    pub fn new(data: T) -> Decoder<T> {
        Self {
            reader: ReverseReader::new(data),
        }
    }

    pub(crate) fn reset(&mut self) {
        self.reader.reset()
    }

    // fn near_next(&mut self, key: &[u8], state: &mut State) -> FstResult<u64> {
    //     let mut frist_k = 0;
    //     if key.len() > 0 {
    //         frist_k = key[0];
    //     }
    //     let mut out: u64 = 0;
    //     let mut greater: bool = false;
    //     loop {
    //         let frist_res = self.loop_state(frist_k, state);
    //         let position = self.reader.get_position();
    //         match frist_res {
    //             Err(e) => match e {
    //                 FstError::Greater => {
    //                     greater = true;
    //                 }
    //                 _ => {
    //                     return Err(FstError::NotFound);
    //                 }
    //             },
    //             Ok(()) => {}
    //         }
    //         out += state.out;
    //         if greater {
    //             loop {
    //                 if state.is_final {
    //                     break;
    //                 }
    //                 self.read_first_state(state)?;
    //                 out += state.out;
    //             }
    //             out += state.final_out;
    //         } else {
    //             let res = self.near_next(&key[1..], state);
    //             match res {
    //                 Ok(_out) => {
    //                     out += _out;
    //                 }
    //                 Err(e) => {
    //                     if !state.is_last {
    //                         self.reader.set_position(position);
    //                         state.reset();
    //                         continue;
    //                     }
    //                     return Err(FstError::NotFound);
    //                 }
    //             }
    //         }
    //         break;
    //     }

    //     Ok(out)
    // }

    // pub(crate) fn near(&mut self, key: &[u8]) -> FstResult<u64> {
    //     let mut state = State::new(0, 0);
    //     state.target = self.reader.get_position() as u64;
    //     self.near_next(key, &mut state)
    // }

    // pub(crate) fn get_prefix(&mut self, key: &[u8]) -> FstResult<u64> {
    //     let mut state = State::new(0, 0);
    //     state.target = self.reader.get_position() as u64;
    //     let mut out: u64 = 0;
    //     let mut f = false;
    //     // let mut last_k: u8 = 0;
    //     for _k in key.iter() {
    //         if let Ok(()) = self.find_target_state(*_k, &mut state) {
    //             out += state.out;
    //             f = true;
    //         } else {
    //             f = false;
    //             //  last_k = *_k;
    //             break;
    //         }
    //     }
    //     if !state.is_final() {
    //         return Err(FstError::NotFound);
    //     }
    //     if !f && state.is_last() {
    //         return Err(FstError::NotFound);
    //     }
    //     //已经遍历所有相同前缀 然后变量下一个词的时候，已经顺序遍历完最后一个字符，并且没有找到匹配

    //     out += state.final_out;
    //     Ok(out)
    // }

    pub(crate) fn get(&mut self, key: &[u8]) -> FstResult<u64> {
        let mut state = State::new(0, 0);
        state.target = self.reader.get_position() as u64;
        let mut out: u64 = 0;
        for _k in key.iter() {
            self.find_target_state(*_k, &mut state)?;
            out += state.out;
        }
        if !state.is_final() {
            return Err(FstError::NotFound);
        }
        out += state.final_out;
        Ok(out)
    }

    // pub(crate) fn next(&mut self, key: &[u8]) -> FstResult<u64> {
    //     let mut state = State::new(0, 0);
    //     state.target = self.reader.get_position() as u64;
    //     let mut out: u64 = 0;
    //     for _k in key.iter() {
    //         self.find_target_state(*_k, &mut state)?;
    //         out += state.out;
    //     }
    //     if !state.is_final() {
    //         return Err(FstError::NotFound);
    //     }
    //     out += state.final_out;
    //     Ok(out)
    // }

    pub(crate) fn read_first_target_state(
        &mut self,
        follow: &State,
        state: &mut State,
    ) -> FstResult<()> {
        if follow.is_final() {
            state._in = END_LABEL;
            state.out = follow.final_out;
            state.flag = BIT_FINAL_STATE;
            if follow.target <= 0 {
                state.flag |= BIT_LAST_STATE;
            } else {
                state.next_state = follow.target;
            }
            state.target = FINAL_END_NODE;
        } else {
            self.read_next_state_from_follow(follow, state)?;
        }
        Ok(())
    }

    pub(crate) fn read_next_state_from_follow(
        &mut self,
        follow: &State,
        state: &mut State,
    ) -> FstResult<()> {
        self.read_next_state(follow.target, state)?;
        if state.flag(ARCS_AS_FIXED_ARRAY) {
            let num_states = state.final_out as usize;
            self.reader.skip_bytes(num_states * 5);
            self.read_state(state)?;
        }

        Ok(())
    }

    fn find_target_state(&mut self, _in: u8, state: &mut State) -> FstResult<()> {
        self.read_next_state(state.target, state)?;
        if state.flag(ARCS_AS_FIXED_ARRAY) {
            // do binary search
            let num_states = state.final_out;
            let mut low = 0;
            let mut high = num_states - 1;
            let start = self.reader.get_position();
            while low <= high {
                let mid = (low + high) >> 1; // (low + high)/2
                self.reader.set_position(start - (mid * 5) as usize);
                let mid_label = self.reader.read_u8()? as i32;
                let cmp = mid_label - _in as i32;
                if cmp < 0 {
                    low = mid + 1;
                } else if cmp > 0 {
                    high = mid - 1;
                } else {
                    let position = self.reader.read_u32::<BigEndian>()? as usize;
                    self.reader.set_position(position);
                    self.read_state(state)?;
                    return Ok(());
                }
            }
            return Err(FstError::NotFound);
        }

        loop {
            if state._in == _in {
                return Ok(());
            } else if state._in > _in {
                return Err(FstError::Greater);
            } else if state.is_last() {
                return Err(FstError::NotFound);
            } else {
                self.read_state(state)?;
            }
        }
    }

    pub(crate) fn read_next_state(&mut self, target: u64, state: &mut State) -> FstResult<()> {
        if target > 0 {
            self.reader.set_position(target as usize);
        }
        self.read_state(state)
    }

    fn read_state(&mut self, state: &mut State) -> FstResult<()> {
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
            //  state.is_stop = true;
            state.target = 0;
        } else {
            if !state.flag(BIT_TAGET_NEXT) && !state.flag(ARCS_AS_FIXED_ARRAY) {
                let (v, _) = self.reader.read_vu64::<Binary>();
                state.target = v;
            } else {
                state.target = self.reader.get_position() as u64;
            }
        }
        // if state.flag(BIT_LAST_STATE) {
        //     state.is_last = true;
        // }
        // if state.flag(BIT_FINAL_STATE) {
        //     state.is_final = true;
        // }
        state.next_state = self.reader.get_position() as u64;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {}
}
