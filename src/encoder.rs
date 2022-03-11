use crate::builder::UnCompiledNode;
use anyhow::Result;
use std::io::{Seek, Write};

const MSB: u8 = 0b1000_0000;

pub const BIT_FINAL_ARC: u8 = 1 << 0;
pub const BIT_LAST_ARC: u8 = 1 << 1;
pub const BIT_TAGET_NEXT: u8 = 1 << 2;
pub const BIT_STOP_NODE: u8 = 1 << 3;
pub const BIT_ARC_HAS_OUPPUT: u8 = 1 << 4;
pub const BIT_ARC_HAS_FINAL_OUTPUT: u8 = 1 << 5;
pub const BIT_TARGET_DELTA: u8 = 1 << 6;

const NO_OUTPUT: u64 = 0;

pub struct Encoder<W: Write> {
    writer: W,
    last_forzen_node: u64,
    node_count: u64,
    position: u64,
    buffer: Vec<u8>,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Encoder<W> {
        Self {
            writer: w,
            last_forzen_node: 0,
            node_count: 0,
            position: 0,
            buffer: vec![0; 10],
        }
    }

    pub fn add_node(&mut self, node: UnCompiledNode) -> Result<u64> {
        for (_i, _a) in node.arcs.iter().rev().enumerate() {
            let mut flag: u8 = 0;
            if _i == 0 {
                flag |= BIT_LAST_ARC;
            }
            if _a.target > 0 {
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
                self.position += self.write_v64(_a.out)?;
            }
            if _a.final_out != NO_OUTPUT {
                flag |= BIT_ARC_HAS_FINAL_OUTPUT;
                self.position += self.write_v64(_a.final_out)?;
            }
            self.position += self.write_byte(_a._in)?;
            self.position += self.write_byte(flag)?;
            self.last_forzen_node = self.position - 1;
        }
        Ok(self.last_forzen_node)
    }

    pub fn write_v64(&mut self, out: u64) -> Result<u64> {
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
        Ok(0)
    }

    fn write_byte(&mut self, b: u8) -> Result<u64> {
        self.writer.write(&[b])?;
        Ok(1)
    }

    fn write_bytes(&mut self, b: &[u8]) -> Result<u64> {
        self.writer.write(b)?;
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
        println!("{:?}", wtr.get_ref().len());
    }

    #[test]
    fn test_slice() {
        let mut v = [1, 2, 3];
        v.reverse();
        assert!(v == [3, 2, 1]);
    }
}
