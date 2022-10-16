# furze

finite state transducers (fst) writen in rust, refer to the Lucene fst implementation.Finite StateTransducers is currently widely used in speech recognition and natural language search, processing and other directions

代码示例
```rust
use furze::Builder;
use furze::FST;
fn main() {
    let mut b = Builder::new(vec![]);
    b.add("cat".as_bytes(), 5);
    b.add("dog".as_bytes(), 10);
    b.add("deep".as_bytes(), 15);
    b.add("logs".as_bytes(), 2);
    b.finish();

    let mut d = FST::load(b.get().to_vec());
    let res = d.find("logs".as_bytes());
    match res {
        Ok(v) => {
            println!("out:{}", v);
        }
        Err(e) => {
            println!("error:{:?}", e);
        }
    }
}

```