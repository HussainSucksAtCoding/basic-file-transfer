use std::env;
use std::io;
use std::io::Write;
use std::{fs::File, net::TcpStream};

fn main() {
    let stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    let args: Vec<_> = env::args().collect();

    let filename = &args[1];
    let file_path = File::open(&args[1]).unwrap();

    println!("{file_path:#?}");

    meta_sender(filename, &stream);
    file_sender(file_path, &stream);
}

fn meta_sender(filename: &String, mut stream: &TcpStream) {
    let name = format!("{filename}\n");
    stream.write_all(name.as_bytes()).unwrap();
}

fn file_sender(mut file: File, mut stream: &TcpStream) {
    io::copy(&mut file, &mut stream).unwrap();
}
