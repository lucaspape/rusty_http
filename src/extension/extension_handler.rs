use std::net::TcpStream;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::common::status::HTTPStatus;

#[derive(Clone)]
pub struct ExtensionHandler {
    pub request: fn(
        args: Vec<String>,
        location: &str,
        stream: &TcpStream,
        request: &HTTPRequest,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool) -> bool,
    
    pub args: Vec<String>
}