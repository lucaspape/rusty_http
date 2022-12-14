use std::net::TcpStream;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::http::location::HTTPLocation;
use crate::common::status::HTTPStatus;
use crate::common::status::HTTPStatus::NotFound;

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
                          body: &Vec<String>,
                          write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
                          write_bytes: fn(&TcpStream, Vec<u8>,) -> bool
    ) -> bool {
        let mut location: Option<&HTTPLocation> = None;
        let mut priority: usize = 0;

        for l in self.locations.iter() {
            let (can_handle, p) = l.can_handle(request.path.as_str());

            let empty = if let None = location {
                true
            } else {
                false
            };

            if can_handle && (empty || p > priority) {
                location = Some(l);
                priority = p;
            }
        }

        if let None = location {
            let msg = "no location";

            if !write_header(stream, NotFound, MimeType::Plain, msg.len(), None) {
                return false;
            }

            return write_bytes(stream, Vec::from(String::from("no location").as_bytes()));
        }

        location.unwrap().handle_request(stream, request, body, write_header, write_bytes)
    }
}