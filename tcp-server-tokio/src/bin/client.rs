use std::{io, net::TcpStream};

fn main() {
    // let address = "127.0.0.1:8080";
    // let tcp_stream = TcpStream::connect(address)?;

    let a = 1;
    let b = &a;
    println!("a {}", b)
}
