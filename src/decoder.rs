use crate::builder::Arc;
use crate::encoder::Encoder;

const DROP_MSB: u8 = 0b0111_1111;
const MSB: u8 = 0b1000_0000;
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
        for _k in key.iter() {}
        Some(0)
    }

    fn find_target_arc(&mut self, _in: u8) -> Option<Arc> {
        let flag: u8 = self.read_byte();
        None
    }

    fn read_byte(&mut self) -> u8 {
        self.reader.read_byte()
    }

    fn read_v_u64(&mut self) -> Option<(u64, usize)> {
        let mut result: u64 = 0;
        let mut shift = 0;
        let mut success = false;
        loop {
            let b = self.read_byte();
            let msb_dropped = b & DROP_MSB;
            result |= (msb_dropped as u64) << shift;
            shift += 7;
            if b & MSB == 0 || shift > (9 * 7) {
                success = b & MSB == 0;
                break;
            }
        }
        if success {
            Some((result, shift / 7 as usize))
        } else {
            None
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
