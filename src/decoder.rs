use crate::builder::Arc;

struct ReverseReader {
    data: Vec<u8>,
    i: usize,
}

impl ReverseReader {
    fn new(data: Vec<u8>) -> ReverseReader {
        Self {
            data: data,
            i: len(data) - 1,
        }
    }

    fn read_byte(&self) -> u8 {
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

    fn read_v_u64(&self) -> u64 {
        let mut b = self.read_byte();
        if (b >= 0) {
            return b;
        }
        let mut i = b & 0x7F;
        b = self.read_byte();
        i |= (b & 0x7F) << 7;
        if (b >= 0) {
            return i;
        }
        b = self.read_byte();
        i |= (b & 0x7F) << 14;
        if (b >= 0) {
            return i;
        }
        b = self.read_byte();
        i |= (b & 0x7F) << 21;
        if (b >= 0) {
            return i;
        }
        b = self.read_byte();
        // Warning: the next ands use 0x0F / 0xF0 - beware copy/paste errors:
        i |= (b & 0x0F) << 28;
        if ((b & 0xF0) == 0) {
            return i;
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
        wtr.write_v_u64(45 as u8).unwrap();
        let dtr = Decoder::new(wtr.get_ref());
        
    }
}
