use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                response_handler(&stream);
            }
            Err(e) => println!("{e}"),
        }
    }
}

fn response_handler(mut stream: &TcpStream) {
    let mut buf = BufReader::new(stream);

    let mut filename = String::new();
    buf.read_line(&mut filename).unwrap();
    let filename = filename.trim();

    match File::create_new(filename) {
        Ok(mut file) => {
            io::copy(&mut stream, &mut file).unwrap();
            println!("recieved file {filename} succussfully");
        }
        Err(e) => {
            println!("file exists already: \n{e:?}");
            return;
        }
    };
}
