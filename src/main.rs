//use std::net::{TcpListener, TcpStream};
//use std::io::Write;

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::{CStr, CString};

#[repr(C)]
//#[derive(Debug, Copy, Clone)]
pub struct MDBX_env;

pub const MDBX_MAXDATASIZE: ::std::os::raw::c_uint = 2147483647;
pub const MDBX_NOSUBDIR: ::std::os::raw::c_uint = 16384;
pub const MDBX_NOSYNC: ::std::os::raw::c_uint = 65536;
pub const MDBX_RDONLY: ::std::os::raw::c_uint = 131072;
pub const MDBX_NOMETASYNC: ::std::os::raw::c_uint = 262144;
pub const MDBX_WRITEMAP: ::std::os::raw::c_uint = 524288;
pub const MDBX_MAPASYNC: ::std::os::raw::c_uint = 1048576;
pub const MDBX_NOTLS: ::std::os::raw::c_uint = 2097152;
pub const MDBX_NOLOCK__UNSUPPORTED: ::std::os::raw::c_uint = 4194304;
pub const MDBX_NORDAHEAD: ::std::os::raw::c_uint = 8388608;
pub const MDBX_NOMEMINIT: ::std::os::raw::c_uint = 16777216;
pub const MDBX_COALESCE: ::std::os::raw::c_uint = 33554432;
pub const MDBX_LIFORECLAIM: ::std::os::raw::c_uint = 67108864;
pub const MDBX_UTTERLY_NOSYNC: ::std::os::raw::c_uint = 1114112;
pub const MDBX_PAGEPERTURB: ::std::os::raw::c_uint = 134217728;
pub const MDBX_REVERSEKEY: ::std::os::raw::c_uint = 2;
pub const MDBX_DUPSORT: ::std::os::raw::c_uint = 4;
pub const MDBX_INTEGERKEY: ::std::os::raw::c_uint = 8;
pub const MDBX_DUPFIXED: ::std::os::raw::c_uint = 16;
pub const MDBX_INTEGERDUP: ::std::os::raw::c_uint = 32;
pub const MDBX_REVERSEDUP: ::std::os::raw::c_uint = 64;
pub const MDBX_CREATE: ::std::os::raw::c_uint = 262144;
pub const MDBX_NOOVERWRITE: ::std::os::raw::c_uint = 16;
pub const MDBX_NODUPDATA: ::std::os::raw::c_uint = 32;
pub const MDBX_CURRENT: ::std::os::raw::c_uint = 64;
pub const MDBX_RESERVE: ::std::os::raw::c_uint = 65536;
pub const MDBX_APPEND: ::std::os::raw::c_uint = 131072;
pub const MDBX_APPENDDUP: ::std::os::raw::c_uint = 262144;
pub const MDBX_MULTIPLE: ::std::os::raw::c_uint = 524288;
pub const MDBX_TRYTXN: ::std::os::raw::c_uint = 268435456;
pub const MDBX_CP_COMPACT: ::std::os::raw::c_uint = 1;
pub const MDBX_SUCCESS: ::std::os::raw::c_uint = 0;
pub const MDBX_RESULT_FALSE: ::std::os::raw::c_uint = 0;
pub const MDBX_RESULT_TRUE: ::std::os::raw::c_int = -1;
pub const MDBX_KEYEXIST: ::std::os::raw::c_int = -30799;
pub const MDBX_NOTFOUND: ::std::os::raw::c_int = -30798;
pub const MDBX_PAGE_NOTFOUND: ::std::os::raw::c_int = -30797;
pub const MDBX_CORRUPTED: ::std::os::raw::c_int = -30796;
pub const MDBX_PANIC: ::std::os::raw::c_int = -30795;
pub const MDBX_VERSION_MISMATCH: ::std::os::raw::c_int = -30794;
pub const MDBX_INVALID: ::std::os::raw::c_int = -30793;
pub const MDBX_MAP_FULL: ::std::os::raw::c_int = -30792;
pub const MDBX_DBS_FULL: ::std::os::raw::c_int = -30791;
pub const MDBX_READERS_FULL: ::std::os::raw::c_int = -30790;
pub const MDBX_TXN_FULL: ::std::os::raw::c_int = -30788;
pub const MDBX_CURSOR_FULL: ::std::os::raw::c_int = -30787;
pub const MDBX_PAGE_FULL: ::std::os::raw::c_int = -30786;
pub const MDBX_MAP_RESIZED: ::std::os::raw::c_int = -30785;
pub const MDBX_INCOMPATIBLE: ::std::os::raw::c_int = -30784;
pub const MDBX_BAD_RSLOT: ::std::os::raw::c_int = -30783;
pub const MDBX_BAD_TXN: ::std::os::raw::c_int = -30782;
pub const MDBX_BAD_VALSIZE: ::std::os::raw::c_int = -30781;
pub const MDBX_BAD_DBI: ::std::os::raw::c_int = -30780;
pub const MDBX_PROBLEM: ::std::os::raw::c_int = -30779;
pub const MDBX_BUSY: ::std::os::raw::c_int = -30778;
pub const MDBX_LAST_ERRCODE: ::std::os::raw::c_int = -30778;
pub const MDBX_EMULTIVAL: ::std::os::raw::c_int = -30421;
pub const MDBX_EBADSIGN: ::std::os::raw::c_int = -30420;
pub const MDBX_WANNA_RECOVERY: ::std::os::raw::c_int = -30419;
pub const MDBX_EKEYMISMATCH: ::std::os::raw::c_int = -30418;
pub const MDBX_TOO_LARGE: ::std::os::raw::c_int = -30417;
pub const MDBX_THREAD_MISMATCH: ::std::os::raw::c_int = -30416;
pub const MDBX_TBL_DIRTY: ::std::os::raw::c_uint = 1;
pub const MDBX_TBL_STALE: ::std::os::raw::c_uint = 2;
pub const MDBX_TBL_NEW: ::std::os::raw::c_uint = 4;
pub const MDBX_DBG_ASSERT: ::std::os::raw::c_uint = 1;
pub const MDBX_DBG_PRINT: ::std::os::raw::c_uint = 2;
pub const MDBX_DBG_TRACE: ::std::os::raw::c_uint = 4;
pub const MDBX_DBG_EXTRA: ::std::os::raw::c_uint = 8;
pub const MDBX_DBG_AUDIT: ::std::os::raw::c_uint = 16;
pub const MDBX_DBG_JITTER: ::std::os::raw::c_uint = 32;
pub const MDBX_DBG_DUMP: ::std::os::raw::c_uint = 64;

pub type MDBX_dbi = u32;

pub struct iovec {
    pub iov_base: *mut ::std::os::raw::c_void,
    pub iov_len: usize,
}

pub type MDBX_val = iovec;

pub struct MDBX_txn {}

pub struct MDBX_cursor {}


/*fn main() {
    let listener = TcpListener::bind("127.0.0.1:32989").unwrap();
    for result in listener.incoming() {
        match result {
            Ok(mut stream) => {
                let mut buf = [0; 10];
                let len = stream.peek(&mut buf).expect("peek failed");
                println!("Client with addr {}", len);
                stream.write(&buf);
            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }

//        let _ = stream?.write(&[1]);
    }
}*/

#[link(name = "mdbx", kind = "static")]
extern {
    fn mdbx_test() -> ();

    pub fn mdbx_env_create(penv: *mut *mut MDBX_env) -> ::std::os::raw::c_uint;
    pub fn mdbx_strerror(errnum: ::std::os::raw::c_uint)
                         -> *const ::std::os::raw::c_char;

    pub fn mdbx_env_open(env: &MDBX_env,
                         path: *const ::std::os::raw::c_char,
                         flags: ::std::os::raw::c_uint, mode: u8)
                         -> ::std::os::raw::c_uint;

    pub fn mdbx_env_close(env: &MDBX_env);

    pub fn mdbx_txn_begin(env: &MDBX_env, parent: *const MDBX_txn,
                          flags: ::std::os::raw::c_uint,
                          txn: &MDBX_txn) -> ::std::os::raw::c_uint;
}

impl MDBX_env {
    fn new() -> MDBX_env {
       MDBX_env{}
    }

    fn create(&mut self) -> u32 {
        return unsafe {
            let rc = mdbx_env_create(&mut self);
            rc
        }
    }

    fn open(&self, path: &str, flags: u32, mode: u8) -> u32 {
        let cPath = CString::new(path).unwrap();
        return unsafe {
            mdbx_env_open(self, cPath.as_ptr(), flags, mode)
        }
    }

}

impl Drop for MDBX_env {
    fn drop(&mut self) {
        unsafe {
            mdbx_env_close(self);
        }
    }
}

impl MDBX_txn {

    fn new()->MDBX_txn {
        MDBX_txn{}
    }

    fn begin(&self, env: &MDBX_env, flags: u32) -> u32 {

        return unsafe {
            mdbx_txn_begin(env, std::ptr::null(), flags, self)
        }
    }

    fn commit(&self) -> u32 {
        unsafe {

        }
    }
}


fn main() {

    let mut env = MDBX_env::new();
    let mut result = env.create();

    //TODO if


    result = env.open("./db1", MDBX_NOSUBDIR | MDBX_COALESCE | MDBX_LIFORECLAIM, 0664);

    let mut transaction = MDBX_txn::new();

    result = transaction.begin(&env, 0);














    println!("EEE");

    unsafe {


        mdbx_test();
    }

}


//TODO learn compile libdbmx directly, raw pointers check