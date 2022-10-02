use std::net::TcpStream;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::extension::extension_handler::ExtensionHandler;
use crate::common::status::HTTPStatus;

#[derive(Clone)]
pub struct HTTPLocation {
    pub location: String,
    extension_handler: ExtensionHandler
}

impl HTTPLocation {
    pub fn new(location: &str, extension_handler: ExtensionHandler) -> HTTPLocation {
        return HTTPLocation{
            location: String::from(location),
            extension_handler
        }
    }

    pub fn handle_request(&self,
                      stream: &TcpStream,
                      request: &HTTPRequest,
                      body: &Vec<String>,
                      write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
                      write_bytes: fn(&TcpStream, Vec<u8>) -> bool
    ) -> bool {
        (self.extension_handler.request)(self.extension_handler.args.clone(), &*self.location, stream, request, body, write_header, write_bytes)
    }
}