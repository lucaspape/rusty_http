use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::net::TcpStream;
use std::path::Path;
use chrono::{DateTime, Utc};
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::extension::extension::Extension;
use crate::extension::extension_handler::ExtensionHandler;
use crate::common::status::HTTPStatus;
use crate::extension::file::index::generate_index;

pub struct FileExtension {}

impl Extension for FileExtension {
    fn handler(&self) -> ExtensionHandler {
        ExtensionHandler { request: FileExtension::handle }
    }
}

impl FileExtension {
    fn handle(
        location: &str,
        root: &str,
        index: bool,
        stream: &TcpStream,
        request: &HTTPRequest,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool {
        let mut request_path = String::from(&*request.path);

        if request_path.starts_with(location) {
            request_path = request_path.replacen(location, "", 1);
        }

        let file_path = String::from(root) + request_path.as_str();

        let path = Path::new(file_path.as_str());

        if !path.exists() {
            let msg = "No such file or directory";

            if !write_header(stream, HTTPStatus::NotFound, MimeType::Plain, msg.len(), None) {
                return false;
            }

            return write_bytes(stream, Vec::from(msg.as_bytes()));
        }

        return if path.is_dir() {
            if index {
                Self::send_index(stream, request.path.as_str(), file_path.as_str(), write_header, write_bytes)
            } else {
                let msg = "Forbidden";

                if !write_header(stream, HTTPStatus::NotFound, MimeType::Plain, msg.len(), None) {
                    return false;
                }

                write_bytes(stream, Vec::from(msg.as_bytes()))
            }
        } else {
            Self::send_file(stream, request, file_path.as_str(), write_header, write_bytes)
        };
    }

    fn parse_range(request: &HTTPRequest, len: u64) -> (u64, u64) {
        let range = request.range.replacen("bytes=", "", 1);
        let r: Vec<&str> = range.split("-").collect();

        let mut s: u64 = 0;
        let mut e: u64 = len;

        match r[0].parse() {
            Ok(start) => {
                s = start;
            }
            Err(_) => {}
        }

        match r[1].parse() {
            Ok(end) => {
                e = end;

                //TODO this a fix for safari, needs to be checked
                e += 1;
            }
            Err(_) => {}
        }

        return (s, e);
    }

    fn not_modified_since(request: &HTTPRequest, file: &File) -> bool {
        let metadata = file.metadata().unwrap();

        if request.if_modified_since.len() > 0 {
            let modified: DateTime<Utc> = metadata.modified().unwrap().into();

            match DateTime::parse_from_rfc2822(request.if_modified_since.as_str()) {
                Ok(modified_since) => {
                    return modified.timestamp() < modified_since.timestamp();
                }
                Err(_) => {}
            }
        }

        return false;
    }

    fn read_file(stream: &TcpStream, file: &File, start: u64, end: u64, write_bytes: fn(&TcpStream, Vec<u8>) -> bool) -> bool {
        const CAP: usize = 1024 * 128;
        let mut reader = BufReader::with_capacity(CAP, file);
        reader.seek(SeekFrom::Start(start)).unwrap();

        let mut read: u64 = start;

        loop {
            let length = {
                match reader.fill_buf() {
                    Ok(buffer) => {
                        if !write_bytes(stream, Vec::from(buffer)) {
                            return false;
                        }

                        buffer.len()
                    }

                    Err(error) => {
                        println!("{}", error);

                        return false;
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

        return true;
    }

    fn header_range(start: u64, end: u64, len: u64) -> String {
        let mut range = String::from("bytes ");

        range += format!("{}", start).as_str();
        range += "-";
        range += format!("{}", end).as_str();
        range += "/";
        range += format!("{}", len).as_str();

        return String::from("Content-Range: ") + range.as_str();
    }

    fn send_file(
        stream: &TcpStream,
        request: &HTTPRequest,
        file_path: &str,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool {
        let file = File::open(&*file_path);

        return match file {
            Ok(file) => {
                if Self::not_modified_since(request, &file) {
                    let msg = "Not Modified";

                    if !write_header(stream, HTTPStatus::NotModified, MimeType::Plain, msg.len() as usize, None) {
                        return false;
                    }

                    return write_bytes(stream, Vec::from(msg.as_bytes()));
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
                    (start, end) = Self::parse_range(request, metadata.len());

                    headers.push(Self::header_range(start, end, len));

                    len = end - start;

                    header_status = HTTPStatus::PartialContent;
                }

                if !write_header(stream, header_status, header_mime_type, len as usize, Some(headers)) {
                    return false;
                }

                Self::read_file(stream, &file, start, end, write_bytes)
            }

            Err(error) => {
                println!("{}", error);

                let msg = "Internal Server Error";

                if !write_header(stream, HTTPStatus::InternalServerError, MimeType::Plain, msg.len(), None) {
                    return false;
                }

                write_bytes(stream, Vec::from(msg.as_bytes()))
            }
        };
    }

    fn send_index(
        stream: &TcpStream,
        path: &str,
        local_path: &str,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool {
        let index = generate_index(local_path, path);

        if !write_header(stream, HTTPStatus::OK, MimeType::Html, index.len(), None) {
            return false;
        }

        return write_bytes(stream, Vec::from(index.as_bytes()));
    }
}