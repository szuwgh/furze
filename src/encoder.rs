use crate::builder::UnCompiledNode;
use anyhow::Result;
use std::io::Write;

const MSB: u8 = 0b1000_0000;

const BIT_FINAL_ARC: u8 = 1 << 0;
const BIT_LAST_ARC: u8 = 1 << 1;
const BIT_TAGET_NEXT: u8 = 1 << 2;
const BIT_STOP_NODE: u8 = 1 << 3;
const BIT_ARC_HAS_OUPPUT: u8 = 1 << 4;
const BIT_ARC_HAS_FINAL_OUTPUT: u8 = 1 << 5;
const BIT_TARGET_DELTA: u8 = 1 << 6;

const NO_OUTPUT: i64 = 0;

pub struct Encoder<W: Write> {
    writer: W,
    last_forzen_node: u32,
    node_count: u32,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Encoder<W> {
        Self {
            writer: w,
            last_forzen_node: 0,
            node_count: 0,
        }
    }

    fn add_node(&mut self, node: UnCompiledNode) -> Result<u32> {
        let last_arc = node.num_arc - 1;
        for (_i, _a) in node.arcs.iter().enumerate() {
            self.node_count += 1;
            let mut flag: u8 = 0;
            if _i == last_arc {
                flag |= BIT_LAST_ARC;
            }
            if self.last_forzen_node == _a.target {
                flag |= BIT_TAGET_NEXT;
            }
            if _a.out != NO_OUTPUT {
                flag |= BIT_ARC_HAS_OUPPUT;
                //  self.write_v_u64(_a.out)?;
            }
            self.write_byte(_a._in)?;
            self.write_byte(flag)?;
        }
        Ok(self.node_count)
    }

    fn write_v_u64(&mut self, out: u64) -> Result<()> {
        let mut n = out;
        while n >= 0x80 {
            self.write_byte(MSB | (n as u8))?;
            n >>= 7;
        }
        self.write_byte(n as u8)?;
        Ok(())
    }

    fn write_byte(&mut self, b: u8) -> Result<()> {
        self.writer.write(&[b])?;
        Ok(())
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
        wtr.write_byte('b' as u8).unwrap();
        //println!("{:?}", wtr.get_ref());
    }
}
