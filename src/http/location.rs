use std::net::TcpStream;
use std::path::Path;
use regex::Regex;

use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::extension::extension_handler::ExtensionHandler;
use crate::common::status::HTTPStatus;

#[derive(Clone)]
pub struct HTTPLocation {
    location: String,
    root: String,
    extension_handler: ExtensionHandler,
    index_files: Vec<String>
}

impl HTTPLocation {
    pub fn new(location: &str, root: &str, extension_handler: ExtensionHandler, index_files: Vec<String>) -> HTTPLocation {
        return HTTPLocation{
            location: String::from(location),
            root: String::from(root),
            extension_handler,
            index_files
        }
    }

    pub fn can_handle(&self, path: &str) -> (bool, usize) {
        let r = Regex::new(format!(r"{}", self.location).as_str()).unwrap();

        if r.is_match(path) {
            return (true, self.location.len())
        }

        for index_file in self.index_files.clone() {
            let index_file_path = self.root.clone() + "/" + path + "/" + index_file.as_str();

            if Path::new(index_file_path.as_str()).exists() {
                return (true, self.location.len());
            }
        }

        return (false, 0)
    }

    pub fn handle_request(&self,
                      stream: &TcpStream,
                      request: &HTTPRequest,
                      body: &Vec<String>,
                      write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
                      write_bytes: fn(&TcpStream, Vec<u8>) -> bool
    ) -> bool {
        (self.extension_handler.request)(self.extension_handler.args.clone(), &*self.location, &*self.root, self.index_files.clone(), stream, request, body, write_header, write_bytes)
    }
}