use std::slice::SliceIndex;

use crate::builder::Builder;
use crate::bytes::Bytes;
use crate::decoder::Decoder;
use crate::decoder::END_LABEL;
use crate::error::FstResult;
use crate::state::State;
pub struct FST<T: AsRef<[u8]>> {
    data: T,
}

impl<T: AsRef<[u8]>> FST<T> {
    pub fn load(data: T) -> FST<T> {
        Self { data: data }
    }

    pub fn get(&self, key: &[u8]) -> FstResult<u64> {
        let mut decoder = Decoder::new(&self.data);
        decoder.get(key)
    }

    pub fn get_ref(&self) -> &T {
        &self.data
    }

    pub fn iter<'a>(&'a self) -> FstIterator<'a, T> {
        FstIterator::new(self)
    }
}

pub struct FstIterator<'a, T: AsRef<[u8]>> {
    //fst: &'a FST<T>,
    decoder: Decoder<&'a T>,
    states: Vec<State>,
    upto: usize,
    out: Vec<u64>,
    input: Vec<u8>,
}

impl<'a, T: AsRef<[u8]>> FstIterator<'a, T> {
    pub fn new(fst: &'a FST<T>) -> FstIterator<'a, T> {
        let decoder = Decoder::new(&fst.data);
        Self {
            //fst: fst,
            decoder: decoder,
            states: vec![State::new(0, 0); 10],
            upto: 0,
            out: vec![0u64; 10],
            input: vec![0u8; 10],
        }
    }

    fn do_next(&mut self) -> FstResult<()> {
        if self.upto == 0 {
            self.upto = 1;
            let (follow, next) = self.states.split_first_mut().unwrap();
            self.decoder
                .read_first_target_state(follow, next.first_mut().unwrap())?;
        } else {
            while self.states.get(self.upto).unwrap().is_last() {
                self.upto -= 1;
                if self.upto == 0 {
                    return Ok(());
                }
            }
            let next = self.states.get_mut(self.upto).unwrap();
            self.decoder.read_next_state(next.next_state, next)?;
        }
        self.push_first()?;
        Ok(())
    }

    fn push_first(&mut self) -> FstResult<()> {
        loop {
            {
                let state = self.states.get(self.upto).unwrap();
                let (a, b) = self.out.split_at_mut(self.upto);
                let out = b.first_mut().unwrap();
                *out = a.last().unwrap() + state.out;
                if state._in == END_LABEL {
                    break;
                }
                self.input[self.upto] = state._in;
            }
            self.incr();
            let (a, b) = self.states.split_at_mut(self.upto);
            let next = b.first_mut().unwrap(); //self.states.get_mut(self.upto).unwrap();
            let follow = a.last().unwrap();
            self.decoder.read_first_target_state(follow, next)?;
        }
        Ok(())
    }

    fn incr(&mut self) {
        self.upto += 1;
        self.grow();
    }

    fn grow(&mut self) {
        let size = self.upto + 1;
        if size > self.input.len() {
            self.input
                .extend_from_slice(&vec![0u8; size - self.input.len()]);
        }
        if size > self.states.len() {
            self.states
                .extend_from_slice(&vec![State::new(0, 0); size - self.states.len()]);
        }
        if size > self.out.len() {
            self.out
                .extend_from_slice(&vec![0u64; size - self.out.len()]);
        }
    }
}

use core::ptr::NonNull;
#[derive(Clone)]
pub struct Cow {
    ptr: NonNull<u8>,
    len: usize,
}

impl Cow {
    fn from_raw_parts(ptr: *mut u8, length: usize) -> Self {
        unsafe {
            Self {
                ptr: NonNull::new_unchecked(ptr),
                len: length,
            }
        }
    }
}

impl AsRef<[u8]> for Cow {
    fn as_ref(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr() as *const u8, self.len) }
    }
}

impl<'a, T: AsRef<[u8]>> Iterator for FstIterator<'a, T> {
    type Item = (Cow, u64);
    fn next(&mut self) -> Option<Self::Item> {
        if !self.do_next().is_ok() {
            return None;
        }
        if self.upto == 0 {
            return None;
        } else {
            return Some((
                Cow::from_raw_parts(
                    (&self.input[1..self.upto]).as_ptr() as *mut u8,
                    self.upto - 1,
                ),
                self.out[self.upto],
            ));
        }
    }
}

// impl<'a, T: AsRef<[u8]>> FstIterator<'a, T> {
//     pub fn next(&mut self) -> Option<(&[u8], u64)> {
//         if !self.do_next().is_ok() {
//             return None;
//         }
//         if self.upto == 0 {
//             return None;
//         } else {
//             return Some((&self.input[1..self.upto], self.out[self.upto]));
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_to_fst2() {
        let i = 0;
        let mut builder = Builder::new(Bytes::with_capacity(4 * 1024));
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
