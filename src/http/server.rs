use std::{thread};
use std::io::{prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown::Both;

use crate::http::host::HTTPHost;
use crate::http::request::{HTTPRequest};
use crate::http::request::HTTPConnection::KeepAlive;
use crate::http::status::HTTPStatus;

pub struct HTTPServer {
    bind: String,
    default_host: HTTPHost,
    hosts: Vec<HTTPHost>,
}

impl HTTPServer {
    pub fn new(bind: String, default_host: HTTPHost, hosts: Vec<HTTPHost>) -> HTTPServer {
        return HTTPServer {
            bind,
            default_host,
            hosts
        };
    }

    pub fn listen(self) {
        let listener = TcpListener::bind(&self.bind).unwrap();

        println!("Listening on {}", self.bind);

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let default_host = self.default_host.clone();
            let hosts = self.hosts.to_vec();

            thread::spawn(|| {
                Self::handle_stream(stream, default_host, hosts);
            });
        }
    }

    fn handle_stream(mut stream: TcpStream, default_host: HTTPHost,hosts: Vec<HTTPHost>) {
        let buf_reader = BufReader::new(&mut stream);

        let r: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        if r.len() == 0 {
            let _ = stream.shutdown(Both);

            return;
        }

        let request = HTTPRequest::parse(r);

        let mut host: Option<&HTTPHost> = None;

        for h in hosts.iter() {
            if h.server_name == request.host {
                host = Some(h);
                break;
            }
        }

        if host == None {
            host = Some(&default_host);
        }

        let (status, content) = host.unwrap().handle_request(&request);
        stream = Self::send_response(stream, status, content.as_str());

        if request.connection == KeepAlive {
            HTTPServer::handle_stream(stream, default_host, hosts);
        }
    }

    fn send_response(mut stream: TcpStream, status: HTTPStatus, content: &str) -> TcpStream {
        let (status_code, status_name) = status.get();
        let content_length = content.len();

        let header_status = format!("HTTP/1.1 {} {}", status_code, status_name);
        let header_content_length = format!("Content-Length: {content_length}");

        let response = format!(
            "{header_status}\r\n\
             {header_content_length}\r\n\r\n\
             {content}
            "
        );

        stream.write_all(response.as_bytes()).unwrap();

        return stream;
    }
}