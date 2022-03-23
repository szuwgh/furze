use crate::builder::Builder;
use crate::bytes::Bytes;
use crate::decoder::Decoder;
use crate::error::FstResult;

pub struct FST {
    data: Vec<u8>,
}

impl FST {
    pub fn build() -> Builder<Vec<u8>> {
        Builder::new(Bytes::new())
    }

    pub fn load(data: Vec<u8>) -> FST {
        Self { data: data }
    }

    pub fn get(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.find(key)
    }

    pub fn get_first_key(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.near(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_to_fst() {
        let mut builder = FST::build();
        builder.add("aa".as_bytes(), 1).unwrap();
        builder.add("bb".as_bytes(), 2).unwrap();
        builder.add("cc".as_bytes(), 7).unwrap();
        builder.add("zz".as_bytes(), 9).unwrap();
        builder.finish().unwrap();
        let fst = FST::load(builder.bytes().to_vec());

        let res = fst.get("cc".as_bytes());
        match res {
            Ok(v) => println!("res:{}", v),
            Err(e) => println!("e:{}", e),
        }
    }
}
