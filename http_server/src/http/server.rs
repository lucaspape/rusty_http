use std::{thread};
use std::io::{prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown::Both;
use http_common::mime::MimeType;
use http_common::request::HTTPConnection::KeepAlive;
use http_common::request::HTTPRequest;

use crate::http::host::HTTPHost;
use http_common::status::HTTPStatus;

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

        let lines = buf_reader.lines();

        let mut closed = false;

        let r: Vec<_> =
            lines
                .map(|result| {
                    return match result {
                        Ok(result) => {
                            result
                        },
                        Err(_) => {
                            closed = true;
                            String::from("")
                        }
                    }
                })
                .take_while(|line| !line.is_empty())
                .collect();

        if r.len() == 0 || closed {
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

        let stream = host.unwrap().handle_request(Some(stream), &request, Self::write_header, Self::write_bytes);

        if let None = stream {
            return;
        }

        let mut stream = stream.unwrap();
        stream.flush().unwrap();

        if request.connection == KeepAlive {
            HTTPServer::handle_stream(stream, default_host, hosts);
        }
    }

    fn write_header(stream: TcpStream, status: HTTPStatus, content_type: MimeType, content_length: usize, additional: Option<Vec<String>>) -> Option<TcpStream> {
        let (status_code, status_name) = status.get();
        let content_type_name = content_type.get();

        let header_status = format!("HTTP/1.1 {} {}", status_code, status_name);
        let header_content_length = format!("Content-Length: {content_length}");
        let header_content_type = format!("Content-Type: {content_type_name}");

        let mut header = Vec::from(format!(
            "{header_status}\r\n{header_content_length}\r\n{header_content_type}\r\n"));

        if additional != None {
            for h in additional.unwrap().iter() {
                header.extend(h.bytes());
                header.extend("\r\n".bytes());
            }
        }

        header.extend("\r\n".bytes());

        return Self::write_bytes(stream, header);
    }

    fn write_bytes(mut stream: TcpStream, b: Vec<u8>) -> Option<TcpStream> {
        return match stream.write(&b[..]) {
            Ok(_) => {
                Some(stream)
            }
            Err(_) => {
                let _ = stream.shutdown(Both);

                None
            }
        }
    }
}