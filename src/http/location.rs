use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::Path;
use crate::http::mime::MimeType;
use crate::http::request::HTTPRequest;
use crate::http::status::HTTPStatus;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct HTTPLocation {
    pub path: String,
    root: String
}

impl HTTPLocation {
    pub fn new(path: &str, root: &str) -> HTTPLocation {
        return HTTPLocation{
            path: String::from(path),
            root: String::from(root)
        }
    }

    pub fn handle_get(&self, mut stream: Option<TcpStream>, request: &HTTPRequest, write_header: fn(TcpStream, HTTPStatus, MimeType, usize) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let mut path = String::from(&*request.path);

        if path.starts_with(&*self.path) {
            path = path.replace(&*self.path, "");
        }

        let file_path = String::from(self.root.as_str()) + path.as_str();

        let path = Path::new(file_path.as_str());

        if !path.exists() {
            let msg = "No such file or directory";

            stream = write_header(stream.unwrap(), HTTPStatus::NotFound, MimeType::Plain, msg.len());

            if let None = stream {
                return stream;
            }

            stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
            return stream;
        }

        if !path.is_file() {
            let msg = "Is not a file";

            stream = write_header(stream.unwrap(), HTTPStatus::NotFound, MimeType::Plain, msg.len());

            if let None = stream {
                return stream;
            }

            stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
            return stream;
        }

        let file = File::open(&*file_path);

        match file {
            Ok(file) => {
                let len = file.metadata().unwrap().len();
                stream = write_header(stream.unwrap(), HTTPStatus::OK, MimeType::from_file_path(file_path.as_str()), len as usize);

                if let None = stream {
                    return stream;
                }

                const CAP: usize = 1024 * 128;
                let mut reader = BufReader::with_capacity(CAP, file);

                loop {
                    let length = {
                        match reader.fill_buf() {
                            Ok(buffer) => {
                                stream = write_bytes(stream.unwrap(), Vec::from(buffer));

                                if let None = stream {
                                    return stream;
                                }

                                buffer.len()
                            }

                            Err(error) => {
                                println!("{}", error);

                                return None;
                            }
                        }
                    };

                    if length == 0 {
                        break;
                    }
                    reader.consume(length);
                }
            }

            Err(error) => {
                println!("{}", error);

                let msg = "Internal Server Error";

                stream = write_header(stream.unwrap(), HTTPStatus::InternalServerError, MimeType::Plain, msg.len());

                if let None = stream {
                    return stream;
                }

                stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
            }
        };

        return stream;
    }
}