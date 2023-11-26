use http::{Request,Response};
use std::net::TcpStream;

fn main() {
    make_request();
    println!("Hello, world!");
}


fn make_request(){
    let request = Request::builder()
        .uri("https://www.rust-lang.org/")
        .header("User-Agent", "jimbo-jambo-agent/1.0");
     send(request.body(()).unwrap());
    
}

fn send(_req: Request<()>) {
    if let Ok(_stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected!");
    } else{
        println!("failed");
    }

}
