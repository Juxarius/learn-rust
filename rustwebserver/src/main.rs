use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let connection = stream.unwrap();
        handle_connection(connection);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Input stream is mutable because the buffer needs to be mutable

    let mut buffer = [0; 1024];

    // Read from stream and put into buffer
    stream.read(&mut buffer).unwrap();

    // Validating request
    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // Connection string:
    // HTTP-Version Status-code Reason-Phrase CRLF
    // headers CRLF
    // message-body
    //
    // e.g. HTTP/1.1 200 OK\r\n\r\n
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
