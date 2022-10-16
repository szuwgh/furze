use crate::builder::Builder;
use crate::fst::FST;
use std::os::raw::c_void;
use std::slice;

#[no_mangle]
pub extern "C" fn new_fst_builder() -> *mut c_void {
    Box::into_raw(Box::new(FST::build())) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn add_key(arg: *mut c_void, key: *const u8, len: u32, value: u64) -> i32 {
    let b: &mut Builder<Vec<u8>> = &mut *(arg as *mut Builder<Vec<u8>>);
    let k = slice::from_raw_parts(key, len as usize);
    match b.add(k, value) {
        Ok(()) => 0,
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn finish(arg: *mut c_void) -> i32 {
    let b: &mut Builder<Vec<u8>> = &mut *(arg as *mut Builder<Vec<u8>>);
    match b.finish() {
        Ok(()) => 0,
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn bytes(arg: *mut c_void, len: *mut u32, cap: *mut u32) -> *const u8 {
    let b: &mut Builder<Vec<u8>> = &mut *(arg as *mut Builder<Vec<u8>>);
    let data = b.bytes();
    *len = data.len() as u32;
    *cap = data.capacity() as u32;
    data.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn load(key: *mut u8, len: u32, cap: u32) -> *mut c_void {
    let k: Vec<u8> = Vec::from_raw_parts(key, len as usize, cap as usize);
    Box::into_raw(Box::new(FST::load(k))) as *mut c_void
}

#[no_mangle]
pub unsafe extern "C" fn find(arg: *mut c_void, key: *const u8, len: u32) -> i64 {
    let fst: &mut FST = &mut *(arg as *mut FST);
    let k = slice::from_raw_parts(key, len as usize);
    match fst.find(k) {
        Ok(val) => val as i64,
        _ => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_first_key(arg: *mut c_void, key: *const u8, len: u32) -> i64 {
    let fst: &mut FST = &mut *(arg as *mut FST);
    let k = slice::from_raw_parts(key, len as usize);
    match fst.get_first_key(k) {
        Ok(val) => val as i64,
        _ => -1,
    }
}
