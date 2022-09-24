use std::fs::File;
use std::io::{BufReader, ErrorKind, Read};
use crate::http::mime::MimeType;
use crate::http::request::HTTPRequest;
use crate::http::status::HTTPStatus;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct HTTPLocation {
    pub path: String,
    root: String
}

impl HTTPLocation {
    pub fn new(path: &str, root: &str) -> HTTPLocation {
        return HTTPLocation{
            path: String::from(path),
            root: String::from(root)
        }
    }

    pub fn handle_get(&self, request: &HTTPRequest) -> (HTTPStatus, MimeType, Vec<u8>) {
        let mut path = String::from(&*request.path);

        if path.starts_with(&*self.path) {
            path = path.replace(&*self.path, "");
        }

        let file_path = String::from(self.root.as_str()) + path.as_str();

        let file = File::open(&*file_path);

        return match file {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                let mut buffer = Vec::new();
                let result = reader.read_to_end(&mut buffer);

                match result {
                    Ok(_) => {
                        (HTTPStatus::OK, MimeType::from_file_path(file_path.as_str()), buffer)
                    }

                    Err(error) => {
                        println!("{}", error);

                        (HTTPStatus::InternalServerError, MimeType::Plain, Vec::from(String::from("Internal Server Error").as_bytes()))
                    }
                }
            }

            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        (HTTPStatus::NotFound, MimeType::Plain, Vec::from(String::from("No such file or directory").as_bytes()))
                    }
                    _ => {
                        println!("{}", error);

                        (HTTPStatus::InternalServerError, MimeType::Plain, Vec::from(String::from("Internal Server Error").as_bytes()))
                    }
                }
            }
        };
    }
}