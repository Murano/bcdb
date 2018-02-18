extern crate libc;
mod ffi;

use std::ffi::{CString};
use libc::c_void;
use ffi::{MDBX_val};
use std::mem::transmute;


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

    fn put(&mut self, key: ffi::MDBX_val, value: ffi::MDBX_val, flags: u32) -> i32 {
        unsafe {
            let dbi: ffi::MDBX_dbi = *self.mdbx_dbi;
            ffi::mdbx_put(self.mdbx_txn, dbi, &key, &value, flags)
        }
    }

    fn get(&mut self, key: ffi::MDBX_val, value: ffi::MDBX_val, ) -> i32 {
        unsafe {
            let dbi: ffi::MDBX_dbi = *self.mdbx_dbi;
            ffi::mdbx_get(self.mdbx_txn, dbi, &mut key, &mut value)
        }
    }

}

enum Cast {
    I32,
    I64,
    Str
}

enum Data {
    String(String),
    I32(i32),
    I64(i64),
}

struct RawData {
    value: *const u8,
    cast: Cast
}


struct Session {
    mdbx: MDBX
}


impl Session {
    fn new(db_name: &str) -> Session {
        let mut mdbx = MDBX::new();
        let flags = ffi::MDBX_NOSUBDIR | ffi::MDBX_COALESCE | ffi::MDBX_LIFORECLAIM;
        mdbx.create();
        mdbx.open(db_name, flags, 0o664);

        let session = Session{mdbx};
        session
    }

    //TODO key пока int но потом сделать поддержку других типов через Trait
    fn get(&self, key: i32) -> Result<Data, &'static str> {
        unimplemented!();
    }

    fn set(&self, key: i32, data: Data) ->() {
        let mut raw_data = match data {
            Data::String(str_val) => RawData{value: str_val.as_ptr(), cast: Cast::Str},
            Data::I32(i32_val) => RawData{value: &unsafe {transmute::<i32, [u8; 4]>(i32_val)} as * const u8, cast: Cast::I32},
            Data::I64(i64_val) => RawData{value: &unsafe {transmute::<i64, [u8; 8]>(i64_val)} as * const u8, cast: Cast::I64},
        };

        let value_to_insert = &mut raw_data as *mut _ as *mut c_void;
        let mut mutable_key = key;
        let key_to_insert = &mut mutable_key as *mut _ as *mut c_void;

        let value: MDBX_val = MDBX_val {
            iov_len: std::mem::size_of_val(&raw_data),
            iov_base: value_to_insert
        };

        let key: MDBX_val = MDBX_val {
            iov_len: std::mem::size_of_val(&mutable_key),
            iov_base: key_to_insert
        };


        //TODO error handling
        self.mdbx.txn_begin(0); //TODO flags
        self.mdbx.dbi_open("", 0); //TODO flags
        self.mdbx.put(&key, &value, 0); //TODO flags
        self.mdbx.txn_commit();
        self.mdbx.dbi_close(); //TODO implement
    }


}


pub fn debug() ->(){
    unsafe{ffi::print_d()};
}
