use std::fmt;

pub struct HTTPRequest {
    pub(crate) method: HTTPMethod,
    pub(crate) path: String,
    pub(crate) http_version: String,
    pub(crate) host: String,
    pub(crate) user_agent: String,
    pub(crate) accept: String,
    pub(crate) accept_language: String,
    pub(crate) accept_encoding: String,
    pub(crate) connection: HTTPConnection,
    pub(crate) referer: String
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