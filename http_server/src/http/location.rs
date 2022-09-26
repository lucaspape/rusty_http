use std::net::TcpStream;

use http_common::mime::MimeType;
use http_common::request::HTTPRequest;
use http_common::status::HTTPStatus;
use http_extension::Extension;

pub struct HTTPLocation {
    pub path: String,
    root: String,
    index: bool,
    extension: Box<dyn Extension>
}

impl HTTPLocation {
    pub fn new(path: &str, root: &str, index: bool, extension: Box<dyn Extension>) -> HTTPLocation {
        return HTTPLocation{
            path: String::from(path),
            root: String::from(root),
            index,
            extension
        }
    }

    pub fn handle_get(&mut self,
                      mut stream: Option<TcpStream>,
                      request: &HTTPRequest,
                      write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>,
                      write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>
    ) -> Option<TcpStream> {
        return self.extension.handle_request(stream, request, write_header, write_bytes);
    }

    pub fn handle_head(&mut self, stream: Option<TcpStream>, request: &HTTPRequest, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>) -> Option<TcpStream> {
        return self.handle_get(stream, request, write_header, |_, _| {
           return None;
        });
    }
}