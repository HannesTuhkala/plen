use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:30000").unwrap();
    println!("Listening on 127.0.0.1:30000");

    let mut player1_stream = listener.incoming().next().unwrap().unwrap();
    println!("First player connected");

    let mut player2_stream = listener.incoming().next().unwrap().unwrap();
    println!("Second player connected");

    let mut buffer = [0; 512];
    player1_stream.read(&mut buffer).unwrap();
    println!("Received data from player1: {}", String::from_utf8_lossy(&buffer[..]));

    player2_stream.read(&mut buffer).unwrap();
    println!("Received data from player2: {}", String::from_utf8_lossy(&buffer[..]));

    player1_stream.write("100.0 200.0\0".as_bytes()).unwrap();
    player2_stream.write("300.0 50.0\0".as_bytes()).unwrap();
}
