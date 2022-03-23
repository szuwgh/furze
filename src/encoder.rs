use crate::bytes::Clear;
use crate::error::{FstError, FstResult};
use crate::state::UnCompiledNode;
use crate::state::{
    BIT_FINAL_STATE, BIT_LAST_STATE, BIT_STATE_HAS_FINAL_OUTPUT, BIT_STATE_HAS_OUPPUT,
    BIT_STOP_NODE, BIT_TAGET_NEXT, BIT_TARGET_DELTA,
};
use std::io::Write;

const MSB: u8 = 0b1000_0000;

const NO_OUTPUT: u64 = 0;

pub struct Encoder<W>
where
    W: Write + Clear,
{
    writer: W,
    last_forzen_node: u64,
    position: u64,
}

impl<W> Encoder<W>
where
    W: Write + Clear,
{
    pub fn new(w: W) -> Encoder<W> {
        Self {
            writer: w,
            last_forzen_node: 0,
            position: 0,
        }
    }

    pub fn add_node(&mut self, node: UnCompiledNode) -> FstResult<u64> {
        for (_i, _s) in node.states.iter().rev().enumerate() {
            let mut flag: u8 = 0;
            if _i == 0 {
                flag |= BIT_LAST_STATE;
            }
            if _s.is_final {
                flag |= BIT_FINAL_STATE;
            }
            if _s.target > 0 {
                if self.last_forzen_node == _s.target {
                    flag |= BIT_TAGET_NEXT;
                } else {
                    self.position += self.write_v64(_s.target)?;
                }
            } else {
                flag |= BIT_STOP_NODE;
            }
            if _s.out != NO_OUTPUT {
                flag |= BIT_STATE_HAS_OUPPUT;
                self.position += self.write_v64(_s.out)?;
            }
            if _s.final_out != NO_OUTPUT {
                flag |= BIT_STATE_HAS_FINAL_OUTPUT;
                self.position += self.write_v64(_s.final_out)?;
            }
            self.position += self.write_byte(_s._in)?;
            self.position += self.write_byte(flag)?;
            self.last_forzen_node = self.position - 1;
        }
        Ok(self.last_forzen_node)
    }

    pub fn write_v64(&mut self, out: u64) -> FstResult<u64> {
        let mut buffer: [u8; 10] = [0; 10];
        let mut n = out;
        let mut i = 0;
        while n >= 0x80 {
            buffer[i] = MSB | (n as u8);
            i += 1;
            n >>= 7;
        }
        buffer[i] = n as u8;
        i += 1;
        let b = &mut buffer[0..i];
        b.reverse();
        self.write_bytes(b)?;
        Ok(i as u64)
    }

    fn write_byte(&mut self, b: u8) -> FstResult<u64> {
        let size = self
            .writer
            .write(&[b])
            .map_err(|e| FstError::IoWriteFail(e))?;
        Ok(size as u64)
    }

    fn write_bytes(&mut self, b: &[u8]) -> FstResult<u64> {
        let size = self.writer.write(b).map_err(|e| FstError::IoWriteFail(e))?;
        Ok(size as u64)
    }

    fn flush(&mut self) -> FstResult<()> {
        self.writer.flush().map_err(|e| FstError::IoWriteFail(e))
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn reset(&mut self) {
        self.writer.reset()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let v = vec![];
        let mut wtr = Encoder::new(v);
        println!("{:?}", wtr.get_ref().len());
    }

    #[test]
    fn test_slice() {
        let mut v = [1, 2, 3];
        v.reverse();
        assert!(v == [3, 2, 1]);
    }
}
