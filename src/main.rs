use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};

use packet::TcpPacket;

mod packet;

fn main() {
    let listener: TcpListener;
    match TcpListener::bind("127.0.0.1:7878") {
        Ok(v) => listener = v,
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_conn(stream);
    }
}

fn handle_conn(mut stream: TcpStream) {
    let mut buf_reader = BufReader::new(&mut stream);
    let received: &[u8];

    match buf_reader.fill_buf() {
        Ok(v) => received = v,
        Err(err) => {
            println!("Could not unwrap, {}", err);
            return;
        }
    }

    let packet = TcpPacket::from(received);

    println!("Request: {}", packet);
}
