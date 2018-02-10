extern crate libc;
mod ffi;

use std::ffi::{CString};
use libc::c_void;
use ffi::{MDBX_val};

struct Data {
    key: MDBX_val, //TODO ??
    value: MDBX_val
}

impl Data {
    pub fn new<T1, T2>(key: T1, value: T2) -> Data {
        let mut mutable_key = key;
        let mut mutable_value = value;

        let base_key = &mut mutable_key as *mut _ as *mut c_void;
        let base_value = &mut mutable_value as *mut _ as *mut c_void;
        let tst:*mut u32 = unsafe{ std::mem::transmute(&mutable_value)};
        let r = &mut mutable_value as *mut _;
        let ch = r as *mut c_void;

        let data_key: MDBX_val = MDBX_val {
            iov_len: std::mem::size_of_val(&mutable_key),
            iov_base: base_key
        };

        let data_value: MDBX_val = MDBX_val {
            iov_len: std::mem::size_of_val(&mutable_value),
            iov_base: base_value
        };

        Data {key: data_key, value: data_value}
    }

    pub fn from_key<T>(key: T) -> Data {

        let mut mutable_key = key;
        let base_key = &mut mutable_key as *mut _ as *mut c_void;

        let data_key: MDBX_val = MDBX_val {
            iov_len: std::mem::size_of_val(&mutable_key),
            iov_base: base_key
        };

        let mut data_value: MDBX_val = MDBX_val {
            iov_len: 0,
            iov_base: std::ptr::null_mut()
        };

        Data {key: data_key, value: data_value}
    }
}


struct MDBX {
    mdbx_env: *mut ffi::MDBX_env,
    mdbx_dbi: *mut ffi::MDBX_dbi,
    mdbx_txn: *mut ffi::MDBX_txn
}


impl MDBX {
    fn new() -> MDBX {
        MDBX{
            mdbx_env: unsafe {std::mem::uninitialized()},
            mdbx_dbi: &mut 0,
            mdbx_txn: unsafe {std::mem::uninitialized()}
        }
    }

    fn create(&mut self) -> u32 {
        unsafe {
            ffi::mdbx_env_create(&mut self.mdbx_env)
        }
    }

    fn open(&mut self, path: &str, flags: u32, mode: u32) -> u32 {
        let cPath = CString::new(path).unwrap();
        unsafe {
            ffi::mdbx_env_open(self.mdbx_env, cPath.as_ptr(), flags, mode)
        }
    }

    fn txn_begin(&mut self, flags: u32) -> u32 {
        unsafe {
            let parent: *mut ffi::MDBX_txn = std::ptr::null_mut();
            ffi::mdbx_txn_begin(self.mdbx_env, parent, flags, &mut self.mdbx_txn)
        }
    }

    fn dbi_open(&mut self, name: &str, flags: u32) -> i32 {

        unsafe {
            let cName = if name.len() == 0 {
                CString::new(name).unwrap().as_ptr()
            } else {
                std::ptr::null()
            };
            ffi::mdbx_dbi_open(self.mdbx_txn, cName, flags, self.mdbx_dbi)
        }
    }

    fn txn_commit(&mut self) -> u32 {
        let r = unsafe {
            ffi::mdbx_txn_commit(self.mdbx_txn)
        };
        self.mdbx_txn = unsafe {std::mem::uninitialized()};
        r
    }

    fn put(&mut self, data: &Data, flags: u32) -> i32 {
        unsafe {
            let dbi: ffi::MDBX_dbi = *self.mdbx_dbi;
            ffi::mdbx_put(self.mdbx_txn, dbi, &data.key, &data.value, flags)
        }
    }

    fn get(&mut self, data: &mut Data) -> i32 {
        unsafe {
            let dbi: ffi::MDBX_dbi = *self.mdbx_dbi;
            ffi::mdbx_get(self.mdbx_txn, dbi, &mut data.key, &mut data.value)
        }
    }

}


//struct Session {
//    mdbx: MDBX
//}
//
//impl Session {
//    fn new() -> Session {
//
//    }
//
//    fn get() -> () {
//
//    }
//
//
//}

pub fn debug() ->(){
    unsafe{ffi::print_d()};
}
