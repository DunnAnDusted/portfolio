use std::net;
use purple_blox;

fn main() {
    let listener = net::TcpListener::bind("127.0.0.1:7878").unwrap();

    purple_blox::run(listener);
}
