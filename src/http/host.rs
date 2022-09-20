use crate::http::location::HTTPLocation;
use crate::http::request::{HTTPMethod, HTTPRequest};
use crate::http::status::HTTPStatus;
use crate::http::status::HTTPStatus::{NotFound};

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

    pub fn handle_request(&self, request: &HTTPRequest) -> (HTTPStatus, String) {
        let mut location: Option<&HTTPLocation> = None;
        let mut responsibility = 0;

        for l in self.locations.iter() {
            let r = l.responsible(&request.path.as_str()[..request.path.rfind('/').unwrap()+1]);

            if r != -1 {
                if r < responsibility || location == None{
                    location = Some(l);
                    responsibility = r;
                }
            }
        }

        if location == None {
            return (NotFound, String::from("no location"));
        }

        return match request.method {
            HTTPMethod::GET => {
                location.unwrap().handle_get(&request)
            }
        }
    }
}