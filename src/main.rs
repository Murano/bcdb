use std::net::{TcpListener, TcpStream};
use std::io::Write;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:32989").unwrap();
    for result in listener.incoming() {
//        handle_client(stream?);
//        println!("Echo");
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
}
