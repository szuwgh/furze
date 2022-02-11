use anyhow::Result;
use std::io::{Bytes, Write};

pub struct Encoder<W: Write> {
    writer: W,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Encoder<W> {
        Self { writer: w }
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
