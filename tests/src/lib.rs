use fst::{IntoStreamer, Set};
use furze::FST;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp1() -> i64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let ms = since_the_epoch.as_secs() as i64 * 1000i64
        + (since_the_epoch.subsec_nanos() as f64 / 1_000_000.0) as i64;
    ms
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fst() {
        let mut rand_bytes: Vec<(Vec<u8>, u64)> = Vec::with_capacity(128);
        let t1 = timestamp1();
        for x in 0..1024000 {
            let rand_string: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            let x = rand::thread_rng().gen_range(0..65535) as u64;
            rand_bytes.push((rand_string.into_bytes(), x));
        }
        println!("t1:{}", timestamp1() - t1);
        rand_bytes.sort_by(|(a1, a2), (b1, b2)| a1.cmp(b1));
        let t1 = timestamp1();
        let mut builder = FST::build();
        for (x1, x2) in rand_bytes.iter() {
            builder.add(x1, *x2).unwrap();
        }
        builder.finish().unwrap();
        println!("t2:{}", timestamp1() - t1);
        let t1 = timestamp1();
        let fst: FST<'_> = FST::load(builder.bytes());
        for (x1, x2) in rand_bytes.iter() {
            let res = fst.get(x1).unwrap();
            //println!("x2:{},res:{}", x2, res);
            assert!(res == *x2);
        }
        println!("t3:{}", timestamp1() - t1);
    }

    use fst::raw::{Builder, Fst};
    #[test]
    fn test_fst2() {
        let mut rand_bytes: Vec<(Vec<u8>, u64)> = Vec::with_capacity(128);
        let t1 = timestamp1();
        for x in 0..1024000 {
            let rand_string: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            let x = rand::thread_rng().gen_range(0..65535) as u64;
            rand_bytes.push((rand_string.into_bytes(), x));
        }
        println!("t1:{}", timestamp1() - t1);
        rand_bytes.sort_by(|(a1, a2), (b1, b2)| a1.cmp(b1));
        let t1 = timestamp1();
        let mut bfst = Builder::memory();
        for (x1, x2) in rand_bytes.iter() {
            bfst.insert(x1, *x2);
        }
        //  bfst.finish().unwrap();
        println!("t2:{}", timestamp1() - t1);
        let t1 = timestamp1();
        let fst = bfst.into_fst();
        for (x1, x2) in rand_bytes.iter() {
            let res = fst.get(x1).unwrap();
            //  fst.search(aut)
            //println!("x2:{},res:{}", x2, res);
            assert!(res.value() == *x2);
        }
        println!("t3:{}", timestamp1() - t1);
    }
}
