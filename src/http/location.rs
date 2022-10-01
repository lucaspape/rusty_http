use std::net::TcpStream;
use crate::extension::extension_handler::ExtensionHandler;
use crate::http::mime::MimeType;
use crate::http::request::HTTPRequest;
use crate::http::status::HTTPStatus;

#[derive(Clone)]
pub struct HTTPLocation {
    pub location: String,
    root: String,
    index: bool,
    extension_handler: ExtensionHandler
}

impl HTTPLocation {
    pub fn new(location: &str, root: &str, index: bool, extension_handler: ExtensionHandler) -> HTTPLocation {
        return HTTPLocation{
            location: String::from(location),
            root: String::from(root),
            index,
            extension_handler
        }
    }

    pub fn handle_get(&self,
                      stream: &TcpStream,
                      request: &HTTPRequest,
                      write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
                      write_bytes: fn(&TcpStream, Vec<u8>) -> bool
    ) -> bool {
        (self.extension_handler.request)(&*self.location, &*self.root, self.index, stream, request, write_header, write_bytes)
    }

    pub fn handle_head(&self,
                       stream: &TcpStream,
                       request: &HTTPRequest,
                       write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool
    ) -> bool{
        return self.handle_get(stream, request, write_header, |_, _| {
           return false;
        });
    }
}