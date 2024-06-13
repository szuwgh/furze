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
    use furze::Builder;

    #[test]
    fn test_fst() {
        let mut rand_bytes: Vec<(Vec<u8>, u64)> = Vec::with_capacity(128);
        let t1 = timestamp1();
        let mut builder = Builder::new(Vec::with_capacity(4 * 1024));
        for _ in 0..1 {
            for x in 0..1024000 {
                let rand_string: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(9)
                    .map(char::from)
                    .collect();
                let x = rand::thread_rng().gen_range(0..65535) as u64;
                rand_bytes.push((rand_string.into_bytes(), x));
            }
            println!("t1:{}", timestamp1() - t1);
            rand_bytes.sort_by(|(a1, a2), (b1, b2)| a1.cmp(b1));
            let t1 = timestamp1();

            for (x1, x2) in rand_bytes.iter() {
                builder.add(x1, *x2).unwrap();
            }
            builder.finish().unwrap();
            println!("t2:{}", timestamp1() - t1);
            let t1 = timestamp1();
            let fst = FST::load(builder.bytes());
            let mut i = 0;
            for (x1, x2) in rand_bytes.iter() {
                let res = fst.get(x1).unwrap();
                // println!(
                //     "i:{},x11:{},x2:{},res:{}",
                //     i,
                //     String::from_utf8_lossy(x1),
                //     x2,
                //     res
                // );
                assert!(res == *x2);
                i += 1;
            }
            println!("t3:{}", timestamp1() - t1);
            builder.reset().unwrap()
        }
    }

    #[test]
    fn test_iter() {
        let mut rand_bytes: Vec<(Vec<u8>, u64)> = Vec::with_capacity(128);
        let t1 = timestamp1();
        let mut builder = Builder::new(Vec::with_capacity(4 * 1024));
        for _ in 0..1 {
            for x in 0..102 {
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

            for (x1, x2) in rand_bytes.iter() {
                builder.add(x1, *x2).unwrap();
            }
            builder.finish().unwrap();
            println!("t2:{}", timestamp1() - t1);
            // for (k, v) in rand_bytes.iter() {
            //     println!("ik:{},v:{}", String::from_utf8_lossy(k), v);
            // }
            let t1 = timestamp1();
            let fst = FST::load(builder.bytes());
            let mut fst_iter = fst.iter();
            let mut i = 0;
            while let Some((k, v)) = fst_iter.next() {
                println!("k:{},v:{}", String::from_utf8_lossy(k.as_ref()), v);
                assert!(
                    String::from_utf8_lossy(&rand_bytes[i].0)
                        == String::from_utf8_lossy(k.as_ref())
                );
                assert!(rand_bytes[i].1 == v);
                i += 1;
                // i += 1;
                // if i > 10 {
                //     break;
                // }
            }

            // for (x1, x2) in rand_bytes.iter() {
            //     let res = fst.get(x1).unwrap();
            //     // println!("x2:{},res:{}", x2, res);
            //     assert!(res == *x2);
            // }
            println!("t3:{}", timestamp1() - t1);
            builder.reset().unwrap()
        }
    }

    use fst::raw::{Builder as fstBuiler, Fst};
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
        let mut bfst = fstBuiler::memory();
        for (x1, x2) in rand_bytes.iter() {
            bfst.insert(x1, *x2);
        }
        //  bfst.finish().unwrap();
        println!("t2:{}", timestamp1() - t1);
        let t1 = timestamp1();
        let fst = bfst.into_fst();
        // for x in fst. {}
        // for (x1, x2) in rand_bytes.iter() {
        //     let res = fst.get(x1).unwrap();
        //     //  fst.search(aut)
        //     //println!("x2:{},res:{}", x2, res);
        //     assert!(res.value() == *x2);
        // }
        println!("t3:{}", timestamp1() - t1);
    }
}
