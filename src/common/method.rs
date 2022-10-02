use std::fmt;

#[derive(PartialEq)]
pub enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl HTTPMethod {
    pub fn get_string(&self) -> &str {
        match self {
            HTTPMethod::GET => {
                "GET"
            },
            HTTPMethod::HEAD => {
                "HEAD"
            },
            HTTPMethod::POST => {
                "POST"
            },
            HTTPMethod::PUT => {
                "PUT"
            },
            HTTPMethod::DELETE => {
                "DELETE"
            },
            HTTPMethod::CONNECT => {
                "CONNECT"
            },
            HTTPMethod::OPTIONS => {
                "OPTIONS"
            },
            HTTPMethod::TRACE => {
                "TRACE"
            },
            HTTPMethod::PATCH => {
                "PATCH"
            }
        }
    }

    pub fn parse(s: &str) -> Option<HTTPMethod> {
        let upper = s.to_uppercase();

        return if upper.contains("GET") {
            Some(HTTPMethod::GET)
        }else if upper.contains("HEAD") {
            Some(HTTPMethod::HEAD)
        }else if upper.contains("POST") {
            Some(HTTPMethod::POST)
        }else if upper.contains("PUT") {
            Some(HTTPMethod::PUT)
        }else if upper.contains("DELETE") {
            Some(HTTPMethod::DELETE)
        }else if upper.contains("CONNECT") {
            Some(HTTPMethod::CONNECT)
        }else if upper.contains("OPTIONS") {
            Some(HTTPMethod::OPTIONS)
        }else if upper.contains("TRACE") {
            Some(HTTPMethod::TRACE)
        }else if upper.contains("PATCH") {
            Some(HTTPMethod::PATCH)
        }else{
            None
        }
    }
}

impl fmt::Debug for HTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_string())
    }
}