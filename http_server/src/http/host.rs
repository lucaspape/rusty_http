use std::net::TcpStream;
use http_common::mime::MimeType;
use http_common::request::{HTTPMethod, HTTPRequest};
use crate::http::location::HTTPLocation;
use http_common::status::HTTPStatus;
use http_common::status::HTTPStatus::{NotFound};

#[derive(Clone)]
#[derive(PartialEq)]
pub struct HTTPHost {
    pub server_name: String,
    locations: Vec<HTTPLocation>
}

impl HTTPHost {
    pub fn new(server_name: &str, locations: Vec<HTTPLocation>) -> HTTPHost {
        return HTTPHost {
            server_name: String::from(server_name),
            locations
        }
    }

    pub fn handle_request(&self, mut stream: Option<TcpStream>, request: &HTTPRequest, write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>, write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>) -> Option<TcpStream> {
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

            stream = write_header(stream.unwrap(), NotFound, MimeType::Plain, msg.len(), None);

            if let None = stream {
                return stream;
            }

            stream = write_bytes(stream.unwrap(), Vec::from(String::from("no location").as_bytes()));
            return stream;
        }

        match request.method {
            HTTPMethod::GET => {
                stream = location.unwrap().handle_get(stream, &request, write_header, write_bytes)
            },
            HTTPMethod::HEAD => {
                stream = location.unwrap().handle_head(stream, &request, write_header)
            }
        }

        return stream;
    }
}