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
}
