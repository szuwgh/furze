use crate::bytes::Clear;
use crate::error::{FstError, FstResult};
use crate::state::UnCompiledNode;
use crate::state::{
    ARCS_AS_FIXED_ARRAY, BIT_FINAL_STATE, BIT_LAST_STATE, BIT_STATE_HAS_FINAL_OUTPUT,
    BIT_STATE_HAS_OUPPUT, BIT_STOP_NODE, BIT_TAGET_NEXT,
};
use byteorder::{BigEndian, WriteBytesExt};
use std::io::Write;
use varintrs::{Binary, WriteBytesVarExt};

const MSB: u8 = 0b1000_0000;

const FIXED_ARRAY_NUM_STATE_DEEP: usize = 10;

const NO_OUTPUT: u64 = 0;

pub struct Encoder<W>
where
    W: Write + Clear,
{
    writer: W,
    last_forzen_node: u64,
    position: u64,
    buffer: [u8; 10],
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
            buffer: [0; 10],
        }
    }

    pub(crate) fn add_node(&mut self, node: UnCompiledNode) -> FstResult<u64> {
        // 当一个节点的边数目太多时，顺序遍历耗时太长，此时改用定长的格式来存储每一条边，在查询时使用二分查找加速查询
        let fixed_able = if node.states.len() > FIXED_ARRAY_NUM_STATE_DEEP {
            true
        } else {
            false
        };
        let mut bytes_per_state: Vec<(u8, u32)> = if fixed_able {
            Vec::with_capacity(node.states.len())
        } else {
            Vec::with_capacity(0)
        };
        for (_i, _s) in node.states.iter().rev().enumerate() {
            let mut flag: u8 = 0;
            if _i == 0 {
                flag |= BIT_LAST_STATE;
            }
            if _s.is_final {
                flag |= BIT_FINAL_STATE;
            }
            if _s.target > 0 {
                if self.last_forzen_node == _s.target && !fixed_able {
                    flag |= BIT_TAGET_NEXT;
                } else {
                    self.position += self.write_vu64(_s.target)?;
                }
            } else {
                flag |= BIT_STOP_NODE;
            }
            if _s.out != NO_OUTPUT {
                flag |= BIT_STATE_HAS_OUPPUT;
                self.position += self.write_vu64(_s.out)?;
            }
            if _s.final_out != NO_OUTPUT {
                flag |= BIT_STATE_HAS_FINAL_OUTPUT;
                self.position += self.write_vu64(_s.final_out)?;
            }
            self.writer.write_u8(_s._in)?;
            self.writer.write_u8(flag)?;

            self.position += 2;
            self.last_forzen_node = self.position;
            if fixed_able {
                bytes_per_state.push((_s._in, (self.last_forzen_node) as u32));
            }
        }
        if fixed_able {
            for (x1, x2) in bytes_per_state.iter() {
                self.write_u32(*x2)?;
                self.writer.write_u8(*x1)?;
            }
            let l = self.write_vu64(bytes_per_state.len() as u64)? as u64;
            self.writer.write_u8(0)?;
            self.writer.write_u8(ARCS_AS_FIXED_ARRAY)?;
            self.position += (bytes_per_state.len() * 5) as u64 + l + 2;
            self.last_forzen_node = self.position;
        }
        Ok(self.last_forzen_node)
    }

    pub(crate) fn write_u32(&mut self, out: u32) -> FstResult<u64> {
        let mut buffer: [u8; 4] = [0; 4];
        buffer.as_mut_slice().write_u32::<BigEndian>(out)?;
        buffer.reverse();
        self.writer.write(&buffer)?;
        Ok(4)
    }

    pub(crate) fn write_vu64(&mut self, out: u64) -> FstResult<u64> {
        let i = self.buffer.as_mut_slice().write_vu64::<Binary>(out)?;
        let b = &mut self.buffer[0..i];
        b.reverse();
        self.writer.write(b)?;
        Ok(i as u64)
    }

    pub(crate) fn write_u8(&mut self, u: u8) -> FstResult<()> {
        self.writer.write_u8(u)?;
        Ok(())
    }

    fn flush(&mut self) -> FstResult<()> {
        self.writer.flush().map_err(|e| FstError::IoWriteFail(e))
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn reset(&mut self) {
        self.writer.reset();
        self.last_forzen_node = 0;
        self.position = 0;
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
        let mut a: [u8; 10] = [0; 10];
        let mut buffer = &mut a[..4];
        buffer.write_u32::<BigEndian>(6);
        //  buffer.reverse();
        println!("{:?}", buffer);
    }
}
