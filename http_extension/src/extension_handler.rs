use std::net::TcpStream;
use http_common::mime::MimeType;
use http_common::request::HTTPRequest;
use http_common::status::HTTPStatus;

#[derive(Clone)]
pub struct ExtensionHandler {
    pub args: Vec<String>,
    pub request: fn(
        args: Vec<String>,
        stream: &TcpStream,
        request: &HTTPRequest,
        write_header: &fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: &fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool,
}

impl Default for ExtensionHandler {
    fn default() -> Self {
        Self {
            args: Vec::new(),
            request: |_, _, _, _, _| {
                false
            },
        }
    }
}