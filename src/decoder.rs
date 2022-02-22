use crate::builder::Arc;

struct ReverseReader {
    data: Vec<u8>,
}

impl ReverseReader {
    fn new(data: Vec<u8>) -> ReverseReader {
        Self { data: data }
    }

    fn read_byte() {}
}

struct Decoder {}

impl Decoder {
    fn find(&self, key: &[u8]) -> Option<u64> {
        Some(0)
    }

    fn find_target_arc(_in: u8) -> Option<Arc> {
        None
    }

    fn read_v_u64() {
        
        byte b = readByte();
    if (b >= 0) return b;
    int i = b & 0x7F;
    b = readByte();
    i |= (b & 0x7F) << 7;
    if (b >= 0) return i;
    b = readByte();
    i |= (b & 0x7F) << 14;
    if (b >= 0) return i;
    b = readByte();
    i |= (b & 0x7F) << 21;
    if (b >= 0) return i;
    b = readByte();
    // Warning: the next ands use 0x0F / 0xF0 - beware copy/paste errors:
    i |= (b & 0x0F) << 28;
    if ((b & 0xF0) == 0) return i;
    }
}
