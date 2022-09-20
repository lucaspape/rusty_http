use std::fmt;

pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub http_version: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub accept_language: String,
    pub accept_encoding: String,
    pub connection: HTTPConnection,
    pub referer: String
}

impl HTTPRequest {
    pub fn new(method: HTTPMethod, path: String, http_version: String, host: String, user_agent: String, accept: String, accept_language: String, accept_encoding: String, connection: HTTPConnection, referer: String) -> HTTPRequest {
        return HTTPRequest{
            method,
            path,
            http_version,
            host,
            user_agent,
            accept,
            accept_language,
            accept_encoding,
            connection,
            referer
        }
    }

    pub fn parse(r: Vec<String>) -> HTTPRequest {
        let mut method: Option<HTTPMethod> = None;
        let mut path: Option<String> = None;
        let mut http_version: Option<String> = None;
        let mut host: Option<String> = None;
        let mut user_agent: String = String::from("");
        let mut accept: String = String::from("");
        let mut accept_language: String = String::from("");
        let mut accept_encoding: String = String::from("");
        let mut connection: HTTPConnection = HTTPConnection::KeepAlive;
        let mut referer: String = String::from("");

        for (i, l) in r.iter().enumerate() {
            if i == 0 {
                let components: Vec<&str> = l.split_whitespace().collect();

                if components.len() == 3 {
                    match components[0] {
                        "GET" => method = Some(HTTPMethod::GET),
                        _ => panic!("unknown http method")
                    }

                    path = Some(String::from(components[1]));
                    http_version = Some(String::from(components[2]));
                }else{
                    panic!("wrong first line length")
                }
            }else{
                let header_host = "Host: ";
                let header_user_agent = "User-Agent: ";
                let header_accept = "Accept: ";
                let header_accept_language = "Accept-Language: ";
                let header_accept_encoding = "Accept-Encoding: ";
                let header_connection = "Connection: ";
                let header_referer = "Referer: ";

                if l.starts_with(header_host) {
                    host = Some(l.replace(header_host, ""));
                } else if l.starts_with(header_user_agent) {
                    user_agent = l.replace(header_user_agent, "");
                } else if l.starts_with(header_accept) {
                    accept = l.replace(header_accept, "");
                } else if l.starts_with(header_accept_language) {
                    accept_language = l.replace(header_accept_language, "");
                } else if l.starts_with(header_accept_encoding) {
                    accept_encoding = l.replace(header_accept_encoding, "");
                } else if l.starts_with(header_connection) {
                    match l.replace(header_connection, "").as_str() {
                        "keep-alive" => {
                            connection = HTTPConnection::KeepAlive
                        },
                        _ => panic!("unknown connection type")
                    }
                } else if l.starts_with(header_referer) {
                    referer = l.replace(header_referer, "");
                }
            }
        }

        return HTTPRequest::new(method.unwrap(), path.unwrap(), http_version.unwrap(), host.unwrap(), user_agent, accept, accept_language, accept_encoding, connection, referer);
    }
}

impl fmt::Debug for HTTPRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("HTTPRequest")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("http_version", &self.http_version)
            .field("host", &self.host)
            .field("user_agent", &self.user_agent)
            .field("accept", &self.accept)
            .field("accept_language", &self.accept_language)
            .field("accept_encoding", &self.accept_encoding)
            .field("connection", &self.connection)
            .field("referer", &self.referer)

            .finish()
    }
}

#[derive(PartialEq)]
pub enum HTTPConnection {
    KeepAlive
}

impl fmt::Debug for HTTPConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HTTPConnection::KeepAlive => write!(f, "Keep-Alive"),
        }
    }
}

pub enum HTTPMethod {
    GET
}

impl fmt::Debug for HTTPMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HTTPMethod::GET => write!(f, "GET"),
        }
    }
}