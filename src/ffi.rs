#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{iovec, c_void, int32_t, uint32_t, c_uchar, c_schar};

pub const MDBX_MAXDATASIZE: uint32_t = 2147483647;
pub const MDBX_NOSUBDIR: uint32_t = 16384;
pub const MDBX_NOSYNC: uint32_t = 65536;
pub const MDBX_RDONLY: uint32_t = 131072;
pub const MDBX_NOMETASYNC: uint32_t = 262144;
pub const MDBX_WRITEMAP: uint32_t = 524288;
pub const MDBX_MAPASYNC: uint32_t = 1048576;
pub const MDBX_NOTLS: uint32_t = 2097152;
pub const MDBX_NOLOCK__UNSUPPORTED: uint32_t = 4194304;
pub const MDBX_NORDAHEAD: uint32_t = 8388608;
pub const MDBX_NOMEMINIT: uint32_t = 16777216;
pub const MDBX_COALESCE: uint32_t = 33554432;
pub const MDBX_LIFORECLAIM: uint32_t = 67108864;
pub const MDBX_UTTERLY_NOSYNC: uint32_t = 1114112;
pub const MDBX_PAGEPERTURB: uint32_t = 134217728;
pub const MDBX_REVERSEKEY: uint32_t = 2;
pub const MDBX_DUPSORT: uint32_t = 4;
pub const MDBX_INTEGERKEY: uint32_t = 8;
pub const MDBX_DUPFIXED: uint32_t = 16;
pub const MDBX_INTEGERDUP: uint32_t = 32;
pub const MDBX_REVERSEDUP: uint32_t = 64;
pub const MDBX_CREATE: uint32_t = 262144;
pub const MDBX_NOOVERWRITE: uint32_t = 16;
pub const MDBX_NODUPDATA: uint32_t = 32;
pub const MDBX_CURRENT: uint32_t = 64;
pub const MDBX_RESERVE: uint32_t = 65536;
pub const MDBX_APPEND: uint32_t = 131072;
pub const MDBX_APPENDDUP: uint32_t = 262144;
pub const MDBX_MULTIPLE: uint32_t = 524288;
pub const MDBX_TRYTXN: uint32_t = 268435456;
pub const MDBX_CP_COMPACT: uint32_t = 1;
pub const MDBX_SUCCESS: int32_t = 0;
pub const MDBX_RESULT_FALSE: uint32_t = 0;
pub const MDBX_RESULT_TRUE: int32_t = -1;
pub const MDBX_KEYEXIST: int32_t = -30799;
pub const MDBX_NOTFOUND: int32_t = -30798;
pub const MDBX_PAGE_NOTFOUND: int32_t = -30797;
pub const MDBX_CORRUPTED: int32_t = -30796;
pub const MDBX_PANIC: int32_t = -30795;
pub const MDBX_VERSION_MISMATCH: int32_t = -30794;
pub const MDBX_INVALID: int32_t = -30793;
pub const MDBX_MAP_FULL: int32_t = -30792;
pub const MDBX_DBS_FULL: int32_t = -30791;
pub const MDBX_READERS_FULL: int32_t = -30790;
pub const MDBX_TXN_FULL: int32_t = -30788;
pub const MDBX_CURSOR_FULL: int32_t = -30787;
pub const MDBX_PAGE_FULL: int32_t = -30786;
pub const MDBX_MAP_RESIZED: int32_t = -30785;
pub const MDBX_INCOMPATIBLE: int32_t = -30784;
pub const MDBX_BAD_RSLOT: int32_t = -30783;
pub const MDBX_BAD_TXN: int32_t = -30782;
pub const MDBX_BAD_VALSIZE: int32_t = -30781;
pub const MDBX_BAD_DBI: int32_t = -30780;
pub const MDBX_PROBLEM: int32_t = -30779;
pub const MDBX_BUSY: int32_t = -30778;
pub const MDBX_LAST_ERRCODE: int32_t = -30778;
pub const MDBX_EMULTIVAL: int32_t = -30421;
pub const MDBX_EBADSIGN: int32_t = -30420;
pub const MDBX_WANNA_RECOVERY: int32_t = -30419;
pub const MDBX_EKEYMISMATCH: int32_t = -30418;
pub const MDBX_TOO_LARGE: int32_t = -30417;
pub const MDBX_THREAD_MISMATCH: int32_t = -30416;
pub const MDBX_TBL_DIRTY: uint32_t = 1;
pub const MDBX_TBL_STALE: uint32_t = 2;
pub const MDBX_TBL_NEW: uint32_t = 4;
pub const MDBX_DBG_ASSERT: uint32_t = 1;
pub const MDBX_DBG_PRINT: uint32_t = 2;
pub const MDBX_DBG_TRACE: uint32_t = 4;
pub const MDBX_DBG_EXTRA: uint32_t = 8;
pub const MDBX_DBG_AUDIT: uint32_t = 16;
pub const MDBX_DBG_JITTER: uint32_t = 32;
pub const MDBX_DBG_DUMP: uint32_t = 64;

#[repr(C)]
pub struct MDBX_env;

pub type MDBX_dbi = u32;

pub type MDBX_val = iovec;

pub struct MDBX_txn {}

pub struct MDBX_cursor {}


////////////////#[link(name = "mdbx", kind = "static")] //TODO вынести в Cargo.toml -> links = "mdbx"
extern "C" {
    pub fn mdbx_env_create(penv: *mut *mut MDBX_env) -> ::std::os::raw::c_uint;
}

extern "C" {
    pub fn mdbx_strerror(errnum: uint32_t) -> *const c_uchar;
}

extern "C" {
    pub fn mdbx_env_open(
        env: *mut MDBX_env,
        path: *const c_schar,
        flags: uint32_t,
        mode: uint32_t) -> uint32_t;
}

extern "C" {
    pub fn mdbx_env_close(env: *mut MDBX_env);
}

extern "C" {
    pub fn mdbx_txn_begin(
        env: *mut MDBX_env,
        parent: *mut MDBX_txn,
        flags: uint32_t,
        txn: *mut *mut MDBX_txn) -> uint32_t;
}

extern "C" {
    pub fn mdbx_dbi_open(
        txn: *mut MDBX_txn,
        name: *const c_schar,
        flags: uint32_t,
        dbi: *mut MDBX_dbi) -> int32_t;
}

extern "C" {
    pub fn mdbx_txn_commit(txn: *mut MDBX_txn) -> uint32_t;
}

extern "C" {
    pub fn mdbx_put(
        txn: *mut MDBX_txn,
        dbi: MDBX_dbi,
        key: *const MDBX_val,
        data: *const MDBX_val,
        flags: uint32_t) -> int32_t;
}

extern "C" {
    pub fn mdbx_get(
        txn: *mut MDBX_txn,
        dbi: MDBX_dbi,
        key: *mut MDBX_val,
        data: *mut MDBX_val) -> int32_t;
}

//FOR FFI TESTS
//extern "C" {
//    pub fn show_values(
//        key: *const MDBX_val,
//        data: *const MDBX_val,
//        scalar: int32_t,
//        str: *mut c_uchar) -> ();
//}
extern "C" {
    pub fn print_d() -> ();
}