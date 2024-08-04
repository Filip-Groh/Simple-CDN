use std::net::TcpListener;

mod http_handler;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        http_handler::handle_connection(stream);
    }
}