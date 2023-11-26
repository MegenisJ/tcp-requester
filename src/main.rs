use std::net::TcpStream;

fn main() {
    make_request();
}


fn make_request(){
    if let Ok(_stream) = TcpStream::connect("127.0.0.1:7878") {
        println!("Connected!");
    } else{
        println!("failed");
    }    
}

