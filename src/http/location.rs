use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
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
        let range = request.range.replacen("byte=", "", 1);
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
            Ok(start) => {
                e = start;
            }
            Err(_) => {

            }
        }

        return (s, e);
    }

    fn send_file(&self, mut stream: Option<TcpStream>, request: &HTTPRequest, file_path: &str, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
        let file = File::open(&*file_path);

        match file {
            Ok(mut file) => {
                let metadata = file.metadata().unwrap();

                if request.if_modified_since.len() > 0 {
                    let modified: DateTime<Utc> = metadata.modified().unwrap().into();

                    match DateTime::parse_from_rfc2822(request.if_modified_since.as_str()) {
                        Ok(modified_since) => {
                            if modified.timestamp() < modified_since.timestamp() {
                                let msg = "Not Modified";

                                stream = write_header(stream.unwrap(), HTTPStatus::NotModified, MimeType::Plain, msg.len() as usize, None);

                                if let None = stream {
                                    return stream;
                                }

                                stream = write_bytes(stream.unwrap(), Vec::from(msg.as_bytes()));
                                return stream
                            }
                        },
                        Err(_) => {}
                    }
                }

                let mut start: u64 = 0;
                let mut end: u64 = metadata.len();

                let len = metadata.len();

                let mut headers: Vec<String> = Vec::new();
                headers.push(String::from("Accept-Ranges: bytes"));

                if request.range.len() > 0 {
                    let (s, e) = self.parse_range(request, metadata.len());
                    start = s;
                    end = e;

                    let mut range = String::from("bytes ");
                    range += format!("{}", start).as_str();
                    range += "-";
                    range += format!("{}", end).as_str();
                    range += "/";
                    range += format!("{}", len).as_str();

                    headers.push(String::from("Content-Range: ") + range.as_str());

                    stream = write_header(stream.unwrap(), HTTPStatus::PartialContent, MimeType::from_file_path(file_path), len as usize, Some(headers));
                }else{
                    stream = write_header(stream.unwrap(), HTTPStatus::OK, MimeType::from_file_path(file_path), len as usize, Some(headers));
                }

                file.seek(SeekFrom::Start(start)).unwrap();

                if let None = stream {
                    return stream;
                }

                const CAP: usize = 1024 * 128;
                let mut reader = BufReader::with_capacity(CAP, file);

                let mut read: u64 = 0;

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

                    read += length as u64;

                    if read >= end {
                        break;
                    }

                    reader.consume(length);
                }
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

    fn send_index(&self, mut stream: Option<TcpStream>, base: &str, path: &str, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
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
                    index += name;
                }
            }else{
                index += "\">";

                index += name;
            }

            index += "</a>";
            index += "</td>\n";


            index += "<td>";

            let metadata = path.metadata().unwrap();

            let created: DateTime<Utc> = metadata.modified().unwrap().into();
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

        stream = write_header(stream.unwrap(), HTTPStatus::OK, MimeType::Html, index.len(), None);

        if let None = stream {
            return stream;
        }

        return write_bytes(stream.unwrap(), Vec::from(index.as_bytes()));
    }
}