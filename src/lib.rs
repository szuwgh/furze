pub mod builder;
mod bytes;
mod decoder;
mod encoder;
pub mod error;
mod fst;
mod state;
pub use builder::Builder;
pub use fst::FstIterator;
pub use fst::FST;

mod tests {
    use super::*;

    #[test]
    fn test_iter() {
        let mut b = Builder::new(vec![]);
        b.add("cat".as_bytes(), 5);
        b.add("deep".as_bytes(), 10);
        b.add("do".as_bytes(), 15);
        b.add("dog".as_bytes(), 2);
        b.add("dogs".as_bytes(), 8);
        b.finish();

        let mut fst = FST::load(b.get().to_vec());
        println!("{:?}", b.get());
        // let res = fst.get("dog".as_bytes());
        // match res {
        //     Ok(v) => {
        //         println!("out:{}", v);
        //     }
        //     Err(e) => {
        //         println!("error:{:?}", e);
        //     }
        // }

        let mut f = fst.iter();
        let mut i = 0;
        while let Some((k, v)) = f.next() {
            println!("k:{},v:{}", String::from_utf8_lossy(k), v);
            i += 1;
            if i > 10 {
                break;
            }
        }
    }
}
