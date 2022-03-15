use crate::builder::Arc;
use crate::encoder::Encoder;
use crate::encoder::{
    BIT_ARC_HAS_FINAL_OUTPUT, BIT_ARC_HAS_OUPPUT, BIT_FINAL_ARC, BIT_LAST_ARC, BIT_STOP_NODE,
    BIT_TAGET_NEXT, BIT_TARGET_DELTA,
};
use crate::error::FstError;
use crate::error::FstResult;

const DROP_MSB: u8 = 0b0111_1111;
const MSB: u8 = 0b1000_0000;
struct ReverseReader {
    i: i32,
    data: Vec<u8>,
}

impl ReverseReader {
    fn new(data: Vec<u8>) -> ReverseReader {
        Self {
            i: (data.len() - 1) as i32,
            data: data,
        }
    }

    fn reset(&mut self) {
        self.i = (self.data.len() - 1) as i32
    }

    fn set_position(&mut self, postion: usize) {
        self.i = postion as i32
    }

    fn read_byte(&mut self) -> FstResult<u8> {
        if self.i < 0 {
            return Err(FstError::Eof);
        }
        let b = self.data[self.i as usize];
        self.i -= 1;
        Ok(b)
    }
}

pub struct Decoder {
    reader: ReverseReader,
}

impl Decoder {
    pub fn new(data: Vec<u8>) -> Decoder {
        Self {
            reader: ReverseReader::new(data),
        }
    }

    pub fn print() {}

    pub fn reset(&mut self) {
        self.reader.reset()
    }

    pub fn near(&mut self, key: &[u8]) -> FstResult<u64> {
        let mut arc = Arc::new(0, 0);
        let mut out: u64 = 0;
        let mut frist: bool = false;

        for _k in key.iter() {
            let v = self.near_target_arc(*_k, &mut arc, frist);
            match v {
                Err(FstError::Greater) => {
                    frist = true;
                    break;
                }
                Err(FstError::NotFound) => {
                    return Err(FstError::NotFound);
                }
                _ => {}
            }
            out += arc.out;
        }
        out += arc.final_out;
        Ok(out)
    }

    fn near_next(&mut self, _in: u8, arc: &mut Arc) {}

    fn near_target_arc(&mut self, _in: u8, arc: &mut Arc, frist: bool) -> FstResult<()> {
        self.read_first_arc(arc)?;
        if frist {
            return Ok(());
        }
        loop {
            if arc._in == _in {
                return Ok(());
            } else if arc._in > _in {
                return Err(FstError::Greater);
            } else if arc.is_last {
                return Err(FstError::NotFound);
            } else {
                self.read_next_arc(arc)?;
            }
        }
    }

    pub fn find(&mut self, key: &[u8]) -> FstResult<u64> {
        let mut arc = Arc::new(0, 0);
        let mut out: u64 = 0;
        for _k in key.iter() {
            self.find_target_arc(*_k, &mut arc)?;
            out += arc.out;
        }
        out += arc.final_out;
        Ok(out)
    }

    fn find_target_arc(&mut self, _in: u8, arc: &mut Arc) -> FstResult<()> {
        self.read_first_arc(arc)?;
        loop {
            println!("in:{},target:{}", arc._in, arc.target);
            if arc._in == _in {
                return Ok(());
            } else if arc.is_last {
                return Err(FstError::NotFound);
            } else {
                self.read_next_arc(arc)?;
            }
        }
    }

    fn read_first_arc(&mut self, arc: &mut Arc) -> FstResult<()> {
        if arc.target > 0 {
            self.reader.set_position(arc.target as usize);
        }
        self.read_next_arc(arc)
    }

    fn read_next_arc(&mut self, arc: &mut Arc) -> FstResult<()> {
        arc.reset();
        arc.flag = self.read_byte()?;
        arc._in = self.read_byte()?;
        if arc.flag(BIT_ARC_HAS_FINAL_OUTPUT) {
            let (v, _) = self.read_v_u64()?;
            arc.final_out = v;
        }

        if arc.flag(BIT_ARC_HAS_OUPPUT) {
            let (v, _) = self.read_v_u64()?;
            arc.out = v;
        }
        if arc.flag(BIT_STOP_NODE) {
            arc.is_stop = true;
        } else {
            if !arc.flag(BIT_TAGET_NEXT) {
                let (v, _) = self.read_v_u64()?;
                arc.target = v;
            }
        }
        if arc.flag(BIT_LAST_ARC) {
            arc.is_last = true;
        }

        Ok(())
    }

    fn read_byte(&mut self) -> FstResult<u8> {
        self.reader.read_byte()
    }

    fn read_v_u64(&mut self) -> FstResult<(u64, usize)> {
        let mut result: u64 = 0;
        let mut shift = 0;
        let mut success = false;
        loop {
            let b = self.read_byte()?;
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;
            if b & MSB == 0 || shift > (9 * 7) {
                success = b & MSB == 0;
                break;
            }
        }
        if success {
            Ok((result, shift / 7 as usize))
        } else {
            Err(FstError::Fail)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let v = vec![];
        let mut wtr = Encoder::new(v);
        wtr.write_v64(45).unwrap();
        wtr.write_v64(466987741).unwrap();
        let mut dtr = Decoder::new(wtr.get_ref().to_vec());
        println!("b:{:?}", dtr.read_v_u64().unwrap());
    }
}
