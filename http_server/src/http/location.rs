use std::net::TcpStream;

use http_common::mime::MimeType;
use http_common::request::HTTPRequest;
use http_common::status::HTTPStatus;
use http_extension::extension_handler::ExtensionHandler;


#[derive(Clone)]
pub struct HTTPLocation {
    pub path: String,
    handler: ExtensionHandler
}

impl PartialEq for HTTPLocation {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl HTTPLocation {
    pub fn new(path: &str, handler: ExtensionHandler) -> HTTPLocation {
        return HTTPLocation{
            path: String::from(path),
            handler
        }
    }

    pub fn handle_get(self,
                      stream: &TcpStream,
                      request: &HTTPRequest,
                      write_header: &fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
                      write_bytes: &fn(&TcpStream, Vec<u8>) -> bool
    ) -> bool {
        (self.handler.request)(self.handler.args, stream, request, write_header, write_bytes)
    }

    pub fn handle_head(self, stream: &TcpStream, request: &HTTPRequest, write_header: &fn(
        &TcpStream,
        HTTPStatus,
        MimeType,
        usize,
        Option<Vec<String>>
    ) -> bool) -> bool {
        let write_bytes: fn(&TcpStream, Vec<u8>) -> bool = |_,_| {
            false
        };

        return self.handle_get(stream, request, write_header, &write_bytes);
    }
}