use std::fmt;
use regex::{RegexBuilder};
use crate::common::connection::HTTPConnection;
use crate::common::method::HTTPMethod;
use crate::common::mime::MimeType;

pub struct HTTPRequest {
    pub method: HTTPMethod,
    pub path: String,
    pub query: String,
    pub http_version: String,
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub accept_language: String,
    pub accept_encoding: String,
    pub connection: HTTPConnection,
    pub referer: String,
    pub if_modified_since: String,
    pub range: String,
    pub content_type: MimeType,
    pub cookie: String,
}

impl HTTPRequest {
    pub fn new(method: HTTPMethod,
               path: String,
               query: String,
               http_version: String,
               host: String,
               user_agent: String,
               accept: String,
               accept_language: String,
               accept_encoding: String,
               connection: HTTPConnection,
               referer: String,
               if_modified_since: String,
               range: String,
               content_type: MimeType,
               cookie: String,
    ) -> HTTPRequest {
        return HTTPRequest {
            method,
            path,
            query,
            http_version,
            host,
            user_agent,
            accept,
            accept_language,
            accept_encoding,
            connection,
            referer,
            if_modified_since,
            range,
            content_type,
            cookie,
        };
    }

    pub fn parse(r: Vec<String>) -> HTTPRequest {
        let mut method: Option<HTTPMethod> = None;
        let mut path: Option<String> = None;
        let mut query: Option<String> = None;
        let mut http_version: Option<String> = None;
        let mut host: Option<String> = None;
        let mut user_agent: Option<String> = None;
        let mut accept: Option<String> = None;
        let mut accept_language: Option<String> = None;
        let mut accept_encoding: Option<String> = None;
        let mut connection: HTTPConnection = HTTPConnection::Close;
        let mut referer: Option<String> = None;
        let mut if_modified_since: Option<String> = None;
        let mut range: Option<String> = None;
        let mut content_type: Option<MimeType> = None;
        let mut cookie: Option<String> = None;

        for (i, l) in r.iter().enumerate() {
            if i == 0 {
                let components: Vec<&str> = l.split_whitespace().collect();

                if components.len() >= 3 {
                    method = HTTPMethod::parse(components[0]);

                    let path_components: Vec<&str> = components[1].split("?").collect();
                    path = Some(String::from(path_components[0]));

                    if path_components.len() > 1 {
                        query = Some(String::from(path_components[1]));
                    } else {
                        query = Some(String::from(""));
                    }

                    http_version = Some(String::from(components[2]));
                } else {
                    println!("{:?}", components);
                    panic!("wrong first line length")
                }
            } else {
                if host == None {
                    host = Self::parse_header("Host: ", l);
                }

                if user_agent == None {
                    user_agent = Self::parse_header("User-Agent: ", l);
                }

                if accept == None {
                    accept = Self::parse_header("Accept: ", l);
                }

                if accept_language == None {
                    accept_language = Self::parse_header("Accept-Language: ", l);
                }

                if accept_encoding == None {
                    accept_encoding = Self::parse_header("Accept-Encoding: ", l);
                }

                if referer == None {
                    referer = Self::parse_header("Referer: ", l);
                }

                if if_modified_since == None {
                    if_modified_since = Self::parse_header("If-Modified-Since: ", l);
                }

                if range == None {
                    range = Self::parse_header("Range: ", l);
                }

                let h_connection = Self::parse_header("Connection: ", l);
                if h_connection != None {
                    match h_connection.unwrap().to_lowercase().as_str() {
                        "keep-alive" => {
                            connection = HTTPConnection::KeepAlive
                        }
                        "close" => {
                            connection = HTTPConnection::Close
                        }
                        _ => panic!("unknown connection type")
                    }
                }

                let h_content_type = Self::parse_header("Content-Type: ", l);
                if h_content_type != None {
                    content_type = MimeType::parse(h_content_type.unwrap().as_str());
                }

                if cookie == None {
                    cookie = Self::parse_header("Cookie: ", l);
                }
            }
        }

        if host == None {
            host = Some(String::from(""))
        }

        if user_agent == None {
            user_agent = Some(String::from(""))
        }

        if accept == None {
            accept = Some(String::from(""))
        }

        if accept_language == None {
            accept_language = Some(String::from(""))
        }

        if accept_encoding == None {
            accept_encoding = Some(String::from(""))
        }

        if referer == None {
            referer = Some(String::from(""))
        }

        if if_modified_since == None {
            if_modified_since = Some(String::from(""))
        }

        if range == None {
            range = Some(String::from(""))
        }

        if let None = content_type {
            content_type = Some(MimeType::Plain)
        }

        if cookie == None {
            cookie = Some(String::from(""))
        }

        return HTTPRequest::new(
            method.unwrap(),
            path.unwrap(),
            query.unwrap(),
            http_version.unwrap(),
            host.unwrap(),
            user_agent.unwrap(),
            accept.unwrap(),
            accept_language.unwrap(),
            accept_encoding.unwrap(),
            connection,
            referer.unwrap(),
            if_modified_since.unwrap(),
            range.unwrap(),
            content_type.unwrap(),
            cookie.unwrap());
    }

    pub fn parse_header(header: &str, line: &String) -> Option<String> {
        let search = Self::search_header(header, line);

        if search == None {
            return None;
        }

        let r = RegexBuilder::new(header)
            .case_insensitive(true)
            .build()
            .unwrap();

        return Some(String::from(r.replace(search.unwrap().as_str(), "")));
    }

    pub fn search_header(s: &str, header: &String) -> Option<String> {
        let s = s.to_uppercase();

        if header.to_uppercase().starts_with(&s) {
            return Some(String::from(header));
        }

        return None;
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