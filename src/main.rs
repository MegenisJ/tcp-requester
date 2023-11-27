use std::{
    io::{Read,Write},
    net::TcpStream
};
fn main() -> std::io::Result<()>{
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    stream.write_all(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    let mut line = String::new();
    let _result = stream.read_to_string(&mut line);
    println!("output {}", line);

    Ok(())
}

