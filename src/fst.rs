use crate::builder::Builder;
use crate::bytes::Bytes;
use crate::decoder::Decoder;
use crate::error::FstResult;

pub struct FST<'a> {
    data: &'a [u8],
}

impl<'a> FST<'a> {
    pub fn build() -> Builder<Vec<u8>> {
        Builder::new(Bytes::new())
    }

    pub fn load(data: &'a [u8]) -> FST<'a> {
        Self { data: data }
    }

    pub fn find(&self, key: &[u8]) -> FstResult<u64> {
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
    fn test_add_to_fst() {
        let mut builder = FST::build();
        builder.add("aa".as_bytes(), 0).unwrap();
        builder.add("bb".as_bytes(), 1).unwrap();
        builder.add("c1".as_bytes(), 1).unwrap();
        builder.add("c2".as_bytes(), 2).unwrap();
        builder.add("c3".as_bytes(), 3).unwrap();
        builder.add("c4".as_bytes(), 4).unwrap();
        builder.add("c5".as_bytes(), 5).unwrap();
        builder.add("c6".as_bytes(), 6).unwrap();
        builder.add("c7".as_bytes(), 7).unwrap();
        builder.add("c8".as_bytes(), 8).unwrap();
        builder.add("c9".as_bytes(), 987711587).unwrap();
        builder.add("ca".as_bytes(), 1087711587).unwrap();
        builder.add("cb".as_bytes(), 1187711587).unwrap();
        builder.add("cc".as_bytes(), 1287711587).unwrap();
        builder.add("cd".as_bytes(), 1387711587).unwrap();
        builder.add("zz".as_bytes(), 1487711587).unwrap();
        builder.finish().unwrap();
        println!("{:?},len:{}", builder.get(), builder.get().len());
        let fst = FST::load(builder.bytes());
        let res = fst.find("c9".as_bytes());
        match res {
            Ok(v) => println!("c9 res:{}", v),
            Err(e) => println!("e:{}", e),
        }
        let res = fst.find("cc".as_bytes());
        match res {
            Ok(v) => println!("cc res:{}", v),
            Err(e) => println!("e:{}", e),
        }

        // builder.reset();
        // builder.add("11".as_bytes(), 1).unwrap();
        // builder.add("22".as_bytes(), 2).unwrap();
        // builder.add("33".as_bytes(), 7).unwrap();
        // builder.add("44".as_bytes(), 9).unwrap();
        // builder.finish().unwrap();
        // let fst = FST::load(builder.bytes());
        // let res = fst.find("cc".as_bytes());
        // match res {
        //     Ok(v) => println!("cc res:{}", v),
        //     Err(e) => println!("e:{}", e),
        // }

        // let res1 = fst.find("44".as_bytes());
        // match res1 {
        //     Ok(v) => println!("44 res:{}", v),
        //     Err(e) => println!("e:{}", e),
        // }
    }
}
