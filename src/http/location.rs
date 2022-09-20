use std::fs;
use std::io::ErrorKind;
use crate::http::request::HTTPRequest;
use crate::http::status::HTTPStatus;

#[derive(Clone)]
#[derive(PartialEq)]
pub struct HTTPLocation {
    path: String,
    root: String
}

impl HTTPLocation {
    pub fn new(path: &str, root: &str) -> HTTPLocation {
        return HTTPLocation{
            path: String::from(path),
            root: String::from(root)
        }
    }

    pub fn responsible(&self, path: &str) -> i32 {
        return if path.starts_with(self.path.as_str()) {
            path.replace(self.path.as_str(), "").len() as i32
        } else {
            -1
        }
    }

    pub fn handle_get(&self, request: &HTTPRequest) -> (HTTPStatus, String) {
        let path = String::from(&self.root) + &*request.path;

        let content = fs::read_to_string(path);

        return match content {
            Ok(content) => {
                (HTTPStatus::OK, content)
            }
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        (HTTPStatus::NotFound, String::from("No such file or directory"))
                    }
                    _ => {
                        println!("{}", error);

                        (HTTPStatus::InternalServerError, String::from("Internal Server Error"))
                    }
                }
            }
        };
    }
}