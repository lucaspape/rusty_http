use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::net::TcpStream;
use std::path::Path;
use chrono::{DateTime, Utc};
use crate::http::index::generate_index;
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

    pub fn handle_get(&self, mut stream: Option<TcpStream>, request: &HTTPRequest, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let mut path = String::from(&*request.path);

        if path.starts_with(&*self.path) {
            path = path.replacen(&*self.path, "", 1);
        }

        let file_path = String::from(self.root.as_str()) + path.as_str();

        let path = Path::new(file_path.as_str());

        if !path.exists() {
            let msg = "No such file or directory";

            stream = write_header(stream.unwrap(), HTTPStatus::NotFound, MimeType::Plain, msg.len(), None);

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

                stream = write_header(stream.unwrap(), HTTPStatus::NotFound, MimeType::Plain, msg.len(), None);

                if let None = stream {
                    return stream;
                }

                stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
                stream
            }
        } else {
            self.send_file(stream, request, file_path.as_str(), write_header, write_bytes)
        }
    }

    pub fn handle_head(&self, stream: Option<TcpStream>, request: &HTTPRequest, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>) -> Option<TcpStream> {
        return self.handle_get(stream, request, write_header, |_, _| {
           return None;
        });
    }

    fn parse_range(&self, request: &HTTPRequest, len: u64) -> (u64, u64) {
        let range = request.range.replacen("bytes=", "", 1);
        let r: Vec<&str> = range.split("-").collect();

        let mut s: u64 = 0;
        let mut e: u64 = len;

        match r[0].parse() {
            Ok(start) => {
                s = start;
            }
            Err(_) => {
            }
        }

        match r[1].parse() {
            Ok(end) => {
                e = end;

                //TODO this a fix for safari, needs to be checked
                e += 1;
            }
            Err(_) => {

            }
        }

        return (s, e);
    }

    fn not_modified_since(&self, request: &HTTPRequest, file: &File) -> bool {
        let metadata = file.metadata().unwrap();

        if request.if_modified_since.len() > 0 {
            let modified: DateTime<Utc> = metadata.modified().unwrap().into();

            match DateTime::parse_from_rfc2822(request.if_modified_since.as_str()) {
                Ok(modified_since) => {
                    return modified.timestamp() < modified_since.timestamp();
                },
                Err(_) => {}
            }
        }

        return false;
    }

    fn read_file(&self, mut stream: Option<TcpStream>, file: &File, start: u64, end: u64, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        const CAP: usize = 1024 * 128;
        let mut reader = BufReader::with_capacity(CAP, file);
        reader.seek(SeekFrom::Start(start)).unwrap();

        let mut read: u64 = start;

        loop {
            let length = {
                match reader.fill_buf() {
                    Ok(buffer) => {
                        stream = write_bytes(stream.unwrap(), Vec::from(buffer));

                        if let None = stream {
                            return None;
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

            read += length as u64;

            if read >= end {
                break;
            }

            reader.consume(length);
        }

        return stream;
    }

    fn header_range(&self, start: u64, end: u64, len: u64) -> String {
        let mut range = String::from("bytes ");

        range += format!("{}", start).as_str();
        range += "-";
        range += format!("{}", end).as_str();
        range += "/";
        range += format!("{}", len).as_str();

        return String::from("Content-Range: ") + range.as_str();
    }

    fn send_file(&self, mut stream: Option<TcpStream>, request: &HTTPRequest, file_path: &str, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let file = File::open(&*file_path);

        match file {
            Ok(file) => {
                if self.not_modified_since(request, &file) {
                    let msg = "Not Modified";

                    stream = write_header(stream.unwrap(), HTTPStatus::NotModified, MimeType::Plain, msg.len() as usize, None);

                    if let None = stream {
                        return stream;
                    }

                    stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
                    return stream
                }

                let metadata = file.metadata().unwrap();

                let mut len = metadata.len();
                let mut start: u64 = 0;
                let mut end: u64 = metadata.len();

                let mut headers: Vec<String> = Vec::new();
                headers.push(String::from("Accept-Ranges: bytes"));

                let header_mime_type = MimeType::from_file_path(file_path);
                let mut header_status = HTTPStatus::OK;

                if request.range.len() > 0 {
                    (start, end) = self.parse_range(request, metadata.len());

                    headers.push(self.header_range(start, end, len));

                    len = end-start;

                    header_status = HTTPStatus::PartialContent;
                }

                stream = write_header(stream.unwrap(), header_status, header_mime_type, len as usize, Some(headers));

                if let None = stream {
                    return stream;
                }

                stream = self.read_file(stream, &file, start, end, write_bytes);
            }

            Err(error) => {
                println!("{}", error);

                let msg = "Internal Server Error";

                stream = write_header(stream.unwrap(), HTTPStatus::InternalServerError, MimeType::Plain, msg.len(), None);

                if let None = stream {
                    return stream;
                }

                stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
            }
        };

        return stream;
    }

    fn send_index(&self, mut stream: Option<TcpStream>, path: &str, local_path: &str, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let index = generate_index(local_path, path);

        stream = write_header(stream.unwrap(), HTTPStatus::OK, MimeType::Html, index.len(), None);

        if let None = stream {
            return stream;
        }

        return write_bytes(stream.unwrap(), Vec::from(index.as_bytes()));
    }
}