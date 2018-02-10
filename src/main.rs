//use std::net::{TcpListener, TcpStream};
//use std::io::Write;

extern crate mdbx;


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








/*
impl Drop for MDBX_env {
    fn drop(&mut self) {
        unsafe {
            mdbx_env_close(self);
        }
    }
}
*/



fn main() {
    mdbx::debug();
   /* let mut mdbx = MDBX::new();
    mdbx.create();
    mdbx.open("./db16", MDBX_NOSUBDIR | MDBX_COALESCE | MDBX_LIFORECLAIM, 0o664);
    mdbx.txn_begin(0);
    let open_result = mdbx.dbi_open("5", 0);

    if open_result != MDBX_SUCCESS {
        panic!("Exited with code; {}", open_result)
    }
*/
//    let data = Data::new(54, 245668);
//
//    unsafe {
//        let s = CString::new("885558").unwrap().into_raw();
//        let a = 5;
////        show_values(&data.key, &data.value, 2, s);
//
//        let mut mutable_key = 5;
//
//        let a = MDBX_val {
//            iov_len: std::mem::size_of_val(&mutable_key),
//            iov_base: &mut mutable_key as *mut _ as *mut c_void
//        };
//
//        let s2 = CString::new("22**28+8").unwrap();
//        let mut s3 = s2.into_raw();
//        let mut s4 = s3 as *mut _ as *mut c_void;
////        let mut s5 = s4 as *mut c_void;
//
//        let st = MDBX_val {
//            iov_len: std::mem::size_of_val(&s4),
//            iov_base: s4
//        };
//
//        show_values(&a, &st, 2, s);
//
//        let b = 5;
//    }

    /*let put_result = mdbx.put(&data, 0);


    let commResult = mdbx.txn_commit();

    mdbx.txn_begin(0);
    mdbx.dbi_open("5", 0);

    let mut rData = Data::from_key(54);
    let get_result = mdbx.get(&mut rData); //MDBX_NOTFOUND :(( проблема таки в get 136 трансформируется в другое число(только в stable)

    println!("result: {}", get_result);*/

//    unsafe {mdbx_test();}
}
//libc::c_void -> libc::c_void::__variant2
//TODO разобраться почему не пишется и врубить дебаг в libdbmx