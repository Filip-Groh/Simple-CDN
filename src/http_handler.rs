use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

enum HTTPMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

struct Request {
    method: HTTPMethod,
    path: String,
    protocol_version: String,
    headers: Vec<String>,
    body: String
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut reader = io::BufReader::new(&mut stream);
    let received: Vec<u8> = reader.fill_buf().unwrap().to_vec();
    reader.consume(received.len());

    let request_as_string = String::from_utf8_lossy(&received[..]);

    let mut request_string = request_as_string.split("\r\n");
    let mut first_line = request_string.next().unwrap().split(" ");
    let method_string = first_line.next().unwrap();

    let mut request_lines:Vec<String> = Vec::new();
    request_string.for_each(|line| {
        request_lines.push(line.to_string())
    });

    let request = Request {
        method: match method_string {
            "GET" => HTTPMethod::GET,
            "POST" => HTTPMethod::POST,
            "PUT" => HTTPMethod::PUT,
            "PATCH" => HTTPMethod::PATCH,
            "DELETE" => HTTPMethod::DELETE,
            _ => panic!("Wrong HTTP method!")
        },
        path: first_line.next().unwrap().to_string(),
        protocol_version: first_line.next().unwrap().to_string(),
        headers: request_lines[1..request_lines.len() - 2].to_vec(),
        body: request_lines.last().unwrap().to_string()
    };

    match request.method {
        HTTPMethod::GET => println!("Method: GET"),
        HTTPMethod::POST => println!("Method: POST"),
        HTTPMethod::PUT => println!("Method: PUT"),
        HTTPMethod::PATCH => println!("Method: PATCH"),
        HTTPMethod::DELETE => println!("Method: DELETE")
    }

    println!("Path: {}", request.path);
    println!("Protocol version: {}", request.protocol_version);

    for header in request.headers {
        println!("Header: {}", header)
    }

    println!("Body: {}", request.body);

    let path: String;
    if request.path.ends_with("/") {
        path = request.path.trim_start_matches("/").to_string() + "index.html";
    } else {
        let splitted_path: Vec<&str> = request.path.split(".").collect();
        if splitted_path.len() > 1 {
            path = request.path.trim_start_matches("/").to_string();
        } else {
            path = request.path.trim_start_matches("/").to_string() + ".html";
        }
    }

    println!("Filepath: {}", path);
    
    let status_line = "HTTP/1.1 200 OK";
    let content =  fs::read(path).unwrap();
    let response_header = format!(
        "{}\r\nContent-Length: {}\r\n\r\n",
        status_line,
        content.len()
    ).into_bytes();
    let response: Vec<u8> = response_header.into_iter().chain(content.into_iter()).collect();
    stream.write(&response).unwrap();
    stream.flush().unwrap();
}