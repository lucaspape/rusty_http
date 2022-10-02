use std::fmt;

#[derive(PartialEq)]
pub enum HTTPConnection {
    KeepAlive,
    Close
}

impl fmt::Debug for HTTPConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HTTPConnection::KeepAlive => write!(f, "Keep-Alive"),
            HTTPConnection::Close => write!(f, "Close"),
        }
    }
}