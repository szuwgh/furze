use crate::builder::Builder;
use crate::decoder::Decoder;
use crate::error::FstResult;

pub struct FST {
    data: Vec<u8>,
}

impl FST {
    pub fn build() -> Builder<Vec<u8>> {
        Builder::new(vec![])
    }

    pub fn load(data: Vec<u8>) -> FST {
        Self { data: data }
    }

    pub fn get(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.find(key)
    }

    pub fn near(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.near(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_to_fst() {}
}
