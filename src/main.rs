use std::{
    io::Read,
    net::{TcpListener, TcpStream},
    thread::{self},
};

fn main() {
    // get a handle to connection
    let listener = TcpListener::bind("localhost:2222").expect("could not start server");
    println!("started listening on port localhost:2222...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => handle_stream(stream),
            Err(err) => eprintln!("error connection to stream: {err}"),
        }

        println!("now im here");
    }
}

fn handle_stream(mut stream: TcpStream) {
    thread::spawn(move || {
        println!("im handling {:?}", stream.local_addr().unwrap().ip());
        let mut s = String::new();
        let _ = stream.read_to_string(&mut s);

        println!("connection closed");
    });
}
