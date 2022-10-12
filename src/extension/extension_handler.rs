use std::net::TcpStream;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::common::status::HTTPStatus;

#[derive(Clone)]
pub struct ExtensionHandler {
    pub request: fn(
        args: Vec<String>,
        location: &str,
        root: &str,
        index_files: Vec<String>,
        stream: &TcpStream,
        request: &HTTPRequest,
        body: &Vec<String>,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool) -> bool,

    pub args: Vec<String>
}