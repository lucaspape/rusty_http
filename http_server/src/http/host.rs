use std::net::TcpStream;
use http_common::mime::MimeType;
use http_common::request::{HTTPMethod, HTTPRequest};
use crate::http::location::HTTPLocation;
use http_common::status::HTTPStatus;
use http_common::status::HTTPStatus::{NotFound};

#[derive(PartialEq, Clone)]
pub struct HTTPHost {
    pub server_name: String,
    locations: Vec<HTTPLocation>,
}

impl HTTPHost {
    pub fn new(server_name: &str, locations: Vec<HTTPLocation>) -> HTTPHost {
        return HTTPHost {
            server_name: String::from(server_name),
            locations,
        };
    }

    pub fn handle_request(&self,
                          stream: &TcpStream,
                          request: &HTTPRequest,
                          write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
                          write_bytes: fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool {
        let mut location: Option<&HTTPLocation> = None;
        let mut responsibility = 0;

        for l in self.locations.iter() {
            if request.path.starts_with(&*l.path) {
                let r = request.path.replace(&*l.path, "").len();

                if r < responsibility || location == None {
                    location = Some(l);
                    responsibility = r
                }
            }
        }

        if location == None {
            let msg = "no location";

            if !write_header(stream, NotFound, MimeType::Plain, msg.len(), None) {
                return false;
            }

            if !write_bytes(stream, Vec::from(String::from("no location").as_bytes())) {
                return false;
            }
        }

        return match request.method {
            HTTPMethod::GET => {
                location.unwrap().clone().handle_get(&stream, &request, &write_header, &write_bytes)
            }
            HTTPMethod::HEAD => {
                location.unwrap().clone().handle_head(&stream, &request, &write_header)
            }
        };
    }
}