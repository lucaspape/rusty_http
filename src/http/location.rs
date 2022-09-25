use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::path::Path;
use chrono::{DateTime, Utc};
use humansize::{DECIMAL, format_size};
use crate::http::mime::MimeType;
use crate::http::request::HTTPRequest;
use crate::http::status::HTTPStatus;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct HTTPLocation {
    pub path: String,
    root: String,
    index: bool
}

impl HTTPLocation {
    pub fn new(path: &str, root: &str, index: bool) -> HTTPLocation {
        return HTTPLocation{
            path: String::from(path),
            root: String::from(root),
            index
        }
    }

    pub fn handle_get(&self, mut stream: Option<TcpStream>, request: &HTTPRequest, write_header: fn(TcpStream, HTTPStatus, MimeType, usize) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let mut path = String::from(&*request.path);

        if path.starts_with(&*self.path) {
            path = path.replacen(&*self.path, "", 1);
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

        return if path.is_dir() {
            if self.index {
                self.send_index(stream, request.path.as_str(), file_path.as_str(), write_header, write_bytes)
            } else {
                let msg = "Forbidden";

                stream = write_header(stream.unwrap(), HTTPStatus::NotFound, MimeType::Plain, msg.len());

                if let None = stream {
                    return stream;
                }

                stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
                stream
            }
        } else {
            self.send_file(stream, file_path.as_str(), write_header, write_bytes)
        }
    }

    fn send_file(&self, mut stream: Option<TcpStream>, file_path: &str, write_header: fn(TcpStream, HTTPStatus, MimeType, usize) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let file = File::open(&*file_path);

        match file {
            Ok(file) => {
                let len = file.metadata().unwrap().len();
                stream = write_header(stream.unwrap(), HTTPStatus::OK, MimeType::from_file_path(file_path), len as usize);

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

    fn send_index(&self, mut stream: Option<TcpStream>, base: &str, path: &str, write_header: fn(TcpStream, HTTPStatus, MimeType, usize) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let paths = fs::read_dir(path).unwrap();

        let mut index = String::from("<html> \n");
        index += "<body>\n";

        index += "<h1> Index of ";
        index += base;
        index += "</h1>\n";
        index += "<hr>\n";

        index += "<table>\n";

        index += "<tr>\n";
        index += "<th> Path </th>\n";
        index += "<th> Created </th>\n";
        index += "<th> Size </th>\n";
        index += "</tr>\n";

        index += "<td>";
        index += "<a href=\"../\">../</a>\n";
        index += "</td>\n";

        for path in paths {
            let path = path.unwrap().path();
            let name = path.file_stem().unwrap().to_str().unwrap();

            index += "<tr>\n";

            index += "<td>";

            index += "<a href=\"";

            if base != "/" {
                index += base;
                index += "/";
            }

            index += name;

            if path.is_file() {
                let ext = path.extension();

                if ext != None {
                    let e = ext.unwrap().to_str().unwrap();

                    index += ".";
                    index += e;
                    index += "\">";

                    index += name;
                    index += ".";
                    index += e;
                }else{
                    index += "\">";
                }
            }else{
                index += "\">";

                index += name;
            }

            index += "</a>";
            index += "</td>\n";


            index += "<td>";

            let metadata = path.metadata().unwrap();

            let created: DateTime<Utc> = metadata.created().unwrap().into();
            index += created.format("%Y-%m-%d %T").to_string().as_str();

            index += "</td>\n";

            index += "<td>";

            if path.is_file() {
                let size = metadata.len();

                index += " ";
                index += format_size(size, DECIMAL).as_str();
            }else{
                index += " -";
            }

            index += "</td>\n";
            index += "</tr>\n";
        }

        index += "</table>\n";
        index += "<hr>\n";

        index += "</body>\n";
        index += "</html>";

        stream = write_header(stream.unwrap(), HTTPStatus::OK, MimeType::Html, index.len());

        if let None = stream {
            return stream;
        }

        return write_bytes(stream.unwrap(), Vec::from(index.as_bytes()));
    }
}