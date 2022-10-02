use std::{thread};
use std::io::{prelude::*, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::net::Shutdown::Both;
use crate::common::connection::HTTPConnection::KeepAlive;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;

use crate::http::host::HTTPHost;
use crate::common::status::HTTPStatus;

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
        let read = Self::read_request(&stream);

        if read == None {
            let _ = stream.shutdown(Both);

            return;
        }

        let (header, body) = read.unwrap();

        if header.len() == 0 {
            let _ = stream.shutdown(Both);

            return;
        }

        let request = HTTPRequest::parse(header);
        println!("{:?}", request);

        let mut host: Option<&HTTPHost> = None;

        for h in hosts.iter() {
            if h.server_name == request.host {
                host = Some(h);
                break;
            }
        }

        if let None = host {
            host = Some(&default_host);
        }

        if !host.unwrap().handle_request(&stream, &request, &body, Self::write_header, Self::write_bytes) {
            return
        }

        stream.flush().unwrap();

        if request.connection == KeepAlive {
            HTTPServer::handle_stream(stream, default_host, hosts);
        }
    }

    fn read_request(stream: &TcpStream) -> Option<(Vec<String>, Vec<String>)> {
        let mut header: Vec<String> = Vec::new();
        let mut body: Vec<String> = Vec::new();

        let mut read_header = false;

        const CAP: usize = 1024 * 128;
        let mut reader = BufReader::with_capacity(CAP, stream);

        loop {
            let length = {
                match reader.fill_buf() {
                    Ok(buffer) => {
                        let s = String::from_utf8_lossy(&buffer);

                        for l in s.lines(){
                            if read_header {
                                body.push(String::from(l));
                            }else{
                                if l.is_empty() {
                                    read_header = true;
                                }else{
                                    header.push(String::from(l))
                                }
                            }
                        }

                        buffer.len()
                    }

                    Err(error) => {
                        println!("{}", error);

                        return None;
                    }
                }
            };

            if length == 0 || length < CAP {
                break;
            }

            reader.consume(length);
        }

        return Some((header, body));
    }

    fn write_header(stream: &TcpStream, status: HTTPStatus, content_type: MimeType, content_length: usize, additional: Option<Vec<String>>) -> bool {
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

    fn write_bytes(mut stream: &TcpStream, b: Vec<u8>) -> bool {
        return match stream.write(&b[..]) {
            Ok(_) => {
                true
            }
            Err(_) => {
                let _ = stream.shutdown(Both);

                false
            }
        }
    }
}