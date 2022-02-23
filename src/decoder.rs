use crate::builder::Arc;
use crate::encoder::Encoder;

struct ReverseReader {
    i: usize,
    data: Vec<u8>,
}

impl ReverseReader {
    fn new(data: Vec<u8>) -> ReverseReader {
        Self {
            i: data.len() - 1,
            data: data,
        }
    }

    fn read_byte(&mut self) -> u8 {
        let b = self.data[self.i];
        self.i -= 1;
        b
    }
}

struct Decoder {
    reader: ReverseReader,
}

impl Decoder {
    fn new(data: Vec<u8>) -> Decoder {
        Self {
            reader: ReverseReader::new(data),
        }
    }

    fn find(&self, key: &[u8]) -> Option<u64> {
        Some(0)
    }

    fn find_target_arc(_in: u8) -> Option<Arc> {
        None
    }

    fn read_byte(&mut self) -> u8 {
        self.reader.read_byte()
    }

    fn read_v64(&mut self) -> i64 {
        let mut b = self.read_byte();
        if b >= 0 {
            return b as i64;
        }
        let mut i: i64 = (b & 0x7F) as i64;
        b = self.read_byte();
        i |= ((b & 0x7F) << 7) as i64;
        if b >= 0 {
            return i as i64;
        }
        b = self.read_byte();
        i |= (((b & 0x7F) as i64) << 14) as i64;
        if b >= 0 {
            return i as i64;
        }
        b = self.read_byte();
        i |= (((b & 0x7F) as i64) << 21) as i64;
        if b >= 0 {
            return i as i64;
        }
        b = self.read_byte();
        // Warning: the next ands use 0x0F / 0xF0 - beware copy/paste errors:
        i |= (((b & 0x0F) as i64) << 28) as i64;
        if (b & 0xF0) == 0 {
            return i as i64;
        }
        0
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
        println!("b:{}", dtr.read_v64());
    }
}
