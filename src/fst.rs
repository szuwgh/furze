use crate::builder::Builder;
use crate::bytes::Bytes;
use crate::decoder::Decoder;
use crate::error::FstResult;

pub struct FST<'a> {
    data: &'a [u8],
}

impl<'a> FST<'a> {
    pub fn build() -> Builder<Vec<u8>> {
        Builder::new(Bytes::with_capacity(4 * 1024))
    }

    pub fn load(data: &'a [u8]) -> FST<'a> {
        Self { data: data }
    }

    pub fn get(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.get(key)
    }

    fn get_prefix(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.get_prefix(key)
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
    fn test_add_to_fst2() {
        let mut builder = FST::build();
        builder.add("aa".as_bytes(), 0).unwrap();
        builder.add("bb".as_bytes(), 1).unwrap();
        builder.add("c1".as_bytes(), 1).unwrap();
        builder.add("c2".as_bytes(), 2).unwrap();
        builder.add("c3".as_bytes(), 3).unwrap();
        builder.add("c4".as_bytes(), 4).unwrap();
        builder.add("c5".as_bytes(), 5).unwrap();
        builder.add("c6".as_bytes(), 6).unwrap();
        builder.add("c8".as_bytes(), 8).unwrap();
        builder.add("c9".as_bytes(), 987711587).unwrap();
        builder.add("c911".as_bytes(), 1087711587).unwrap();
        builder.add("c922".as_bytes(), 1187711587).unwrap();
        builder.add("c933".as_bytes(), 1287711587).unwrap();
        builder.add("c944".as_bytes(), 1387711587).unwrap();
        builder.add("z955".as_bytes(), 1487711587).unwrap();
        builder.finish().unwrap();
        println!("{:?},len:{}", builder.get(), builder.get().len());
        let fst = FST::load(builder.bytes());
        let res = fst.get("c9".as_bytes());
        match res {
            Ok(v) => println!("c9 res:{}", v),
            Err(e) => println!("c9 {}", e),
        }
        let res = fst.get("cf5687".as_bytes());
        match res {
            Ok(v) => println!("cf5687 res:{}", v),
            Err(e) => println!("cf5687 {}", e),
        }
    }
}
