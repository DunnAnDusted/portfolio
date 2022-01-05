use std::{
    net,
    io::prelude::*,
    fs, 
    path::Path
};

// Internal value
const SITE_DIR: &str = "purple_blox/site";
const GET: &[u8; 16] = b"GET / HTTP/1.1\r\n";

// Page directories
const INDEX: &str = "index/index.html";
const NOT_FOUND: &str = "not_found/not_found.html";

// Response codes
const OK: &str = "200 OK";
const ERROR_404: &str = "404 NOT FOUND";

pub fn run(listener: net::TcpListener) {
    listener.incoming()
        .filter_map(Result::ok)
        .for_each(handle_connection)
}

fn handle_connection(mut stream: net::TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    
    let (status, filepath) = if buffer.starts_with(GET) {
        (OK, INDEX)
    } else {
        (ERROR_404, NOT_FOUND)
    };

    let filepath = Path::new(SITE_DIR).join(filepath);
    let contents = fs::read_to_string(filepath).unwrap();

    let response = format!(
        "HTTPS/1.1 {}\r\nContent-Length: {}\r\n\r\n{}", 
        status, 
        contents.len(), 
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}