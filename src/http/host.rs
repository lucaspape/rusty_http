use std::net::TcpStream;
use crate::common::mime::MimeType;
use crate::common::request::{HTTPRequest};
use crate::http::location::HTTPLocation;
use crate::common::status::HTTPStatus;
use crate::common::status::HTTPStatus::{NotFound};

#[derive(Clone)]
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
            if request.path.starts_with(&*l.location) {
                let r = request.path.replace(&*l.location, "").len();

                let empty = if let None = location {
                    true
                } else {
                    false
                };

                if r < responsibility || empty {
                    location = Some(l);
                    responsibility = r
                }
            }
        }

        if let None = location {
            let msg = "no location";

            if !write_header(stream, NotFound, MimeType::Plain, msg.len(), None) {
                return false;
            }

            return write_bytes(stream, Vec::from(String::from("no location").as_bytes()));
        }

        location.unwrap().handle_request(stream, &request, write_header, write_bytes)
    }
}