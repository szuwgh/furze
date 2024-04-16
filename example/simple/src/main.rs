use furze::Builder;
use furze::FST;
fn main() {
    let mut b = Builder::new(vec![]);
    b.add("cat".as_bytes(), 5);
    b.add("deep".as_bytes(), 10);
    b.add("do".as_bytes(), 15);
    b.add("dog".as_bytes(), 2);
    b.add("dogs".as_bytes(), 8);
    b.finish();

    let mut fst = FST::load(b.get().to_vec());
    let res = fst.get("logs".as_bytes());
    match res {
        Ok(v) => {
            println!("out:{}", v);
        }
        Err(e) => {
            println!("error:{:?}", e);
        }
    }

    let mut f = fst.iter();
    while let Some((k, v)) = f.next() {
        println!("k:{},v:{}", String::from_utf8_lossy(k), v);
    }
}
