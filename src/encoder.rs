use crate::builder::UnCompiledNode;
use anyhow::Result;
use std::io::{Seek, Write};

const MSB: u8 = 0b1000_0000;

const BIT_FINAL_ARC: u8 = 1 << 0;
const BIT_LAST_ARC: u8 = 1 << 1;
const BIT_TAGET_NEXT: u8 = 1 << 2;
const BIT_STOP_NODE: u8 = 1 << 3;
const BIT_ARC_HAS_OUPPUT: u8 = 1 << 4;
const BIT_ARC_HAS_FINAL_OUTPUT: u8 = 1 << 5;
const BIT_TARGET_DELTA: u8 = 1 << 6;

const NO_OUTPUT: u64 = 0;

pub struct Encoder<W: Write> {
    writer: W,
    last_forzen_node: i64,
    node_count: u64,
    position: u64,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Encoder<W> {
        Self {
            writer: w,
            last_forzen_node: 0,
            node_count: 0,
            position: 0,
        }
    }

    pub fn add_node(&mut self, node: UnCompiledNode) -> Result<i64> {
        for (_i, _a) in node.arcs.iter().rev().enumerate() {
            let mut flag: u8 = 0;
            if _i == 0 {
                flag |= BIT_LAST_ARC;
            }
            let has_target = _a.target > 0;
            if has_target {
                if self.last_forzen_node == _a.target {
                    flag |= BIT_TAGET_NEXT;
                } else {
                    self.position += self.write_v64(_a.target)?;
                }
            } else {
                flag |= BIT_STOP_NODE;
            }
            if _a.out != NO_OUTPUT {
                flag |= BIT_ARC_HAS_OUPPUT;
                self.position += self.write_v64(_a.out as i64)?;
            }
            if _a.final_out != NO_OUTPUT {
                flag |= BIT_ARC_HAS_FINAL_OUTPUT;
                self.position += self.write_v64(_a.final_out as i64)?;
            }
            self.position += self.write_byte(_a._in)?;
            self.position += self.write_byte(flag)?;
            self.last_forzen_node = (self.position - 1) as i64;
        }
        Ok(self.last_forzen_node)
    }

    pub fn write_v64(&mut self, mut out: i64) -> Result<u64> {
        let mut i: u64 = 0;
        while (out & !(0x7F as i64)) != 0 {
            self.write_byte(((out & !(0x7F as i64)) | 0x80 as i64) as u8)?;
            out >>= 7;
        }
        self.write_byte(out as u8)?;
        // let mut n = out;
        // let mut i: u64 = 0;
        // while n >= 0x80 {
        //     self.write_byte(MSB | (n as u8))?;
        //     n >>= 7;
        //     i += 1
        // }
        // i += 1;
        // self.write_byte(n as u8)?;
        Ok(i)
    }

    fn write_byte(&mut self, b: u8) -> Result<u64> {
        self.writer.write(&[b])?;
        Ok(1)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoder() {
        let v = vec![];
        let mut wtr = Encoder::new(v);
        //wtr.write_v_u64('b' as u8).unwrap();
        println!("{:?}", wtr.get_ref().len());
    }
}
