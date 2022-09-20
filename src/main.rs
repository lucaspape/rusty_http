#![feature(io_error_more)]

extern crate core;

mod http_request;

use std::fs;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use http_request::{HTTPRequest, HTTPMethod, HTTPConnection};

const HTML_DIR: &str = "html";

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_stream(stream);
    }
}

fn handle_stream(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let r: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let request = parse_request(r);

    match request.method {
        HTTPMethod::GET => {
            let (code, content) = handle_get(request);
            send_response(stream, code, content.as_str());
            return;
        }
    }
}

fn handle_get(request: HTTPRequest) -> (i16, String){
    let path = String::from(HTML_DIR) + &*request.path;

    let content = fs::read_to_string(path);

    return match content {
        Ok(content) => {
            (200, content)
        }
        Err(error) => {
            match error.kind() {
                ErrorKind::IsADirectory => {
                    (400, String::from("Is a directory"))
                },
                ErrorKind::NotFound => {
                    (404, String::from("No such file or directory"))
                }
                _ => {
                    println!("{}", error);

                    (500, String::from("Internal Server Error"))
                }
            }
        }
    }
}

fn parse_request(r: Vec<String>) -> HTTPRequest {
    let mut method: Option<HTTPMethod> = None;
    let mut path: Option<String> = None;
    let mut http_version: Option<String> = None;
    let mut host: Option<String> = None;
    let mut user_agent: Option<String> = None;
    let mut accept: Option<String> = None;
    let mut accept_language: Option<String> = None;
    let mut accept_encoding: Option<String> = None;
    let mut connection: Option<HTTPConnection> = None;
    let mut referer: Option<String> = None;

    for (i, l) in r.iter().enumerate() {
        if i == 0 {
            let components: Vec<&str> = l.split_whitespace().collect();

            if components.len() == 3 {
                match components[0] {
                    "GET" => method = Some(HTTPMethod::GET),
                    _ => panic!("unknown http method")
                }

                path = Some(String::from(components[1]));
                http_version = Some(String::from(components[2]));
            }else{
                panic!("wrong first line length")
            }
        }else{
            let header_host = "Host: ";
            let header_user_agent = "User-Agent: ";
            let header_accept = "Accept: ";
            let header_accept_language = "Accept-Language: ";
            let header_accept_encoding = "Accept-Encoding: ";
            let header_connection = "Connection: ";
            let header_referer = "Referer: ";

            if l.starts_with(header_host) {
                host = Some(l.replace(header_host, ""));
            } else if l.starts_with(header_user_agent) {
                user_agent = Some(l.replace(header_user_agent, ""));
            } else if l.starts_with(header_accept) {
                accept = Some(l.replace(header_accept, ""));
            } else if l.starts_with(header_accept_language) {
                accept_language = Some(l.replace(header_accept_language, ""));
            } else if l.starts_with(header_accept_encoding) {
                accept_encoding = Some(l.replace(header_accept_encoding, ""));
            } else if l.starts_with(header_connection) {
                match l.replace(header_connection, "").as_str() {
                    "keep-alive" => {
                        connection = Some(HTTPConnection::KeepAlive)
                    },
                    _ => panic!("unknown connection type")
                }
            } else if l.starts_with(header_referer) {
                referer = Some(l.replace(header_referer, ""));
            }
        }
    }

    if referer == None {
        referer = Some(String::from(""));
    }

    return HTTPRequest::new(method.unwrap(), path.unwrap(), http_version.unwrap(), host.unwrap(), user_agent.unwrap(), accept.unwrap(), accept_language.unwrap(), accept_encoding.unwrap(), connection.unwrap(), referer.unwrap());
}

fn send_response(mut stream: TcpStream, code: i16, content: &str) {
    let status_name = "OK";
    let content_length = content.len();

    let header_status = format!("HTTP/1.1 {} {}", code, status_name);
    let header_content_length = format!("Content-Length: {content_length}");

    let response = format!(
        "{header_status}\r\n\
         {header_content_length}\r\n\r\n\
         {content}
        "
    );

    stream.write_all(response.as_bytes()).unwrap();
}