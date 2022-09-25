pub enum HTTPStatus {
    OK,
    NotModified,
    NotFound,
    InternalServerError,
}

impl HTTPStatus {
    pub fn get(self) -> (i16, String) {
        return match self {
            HTTPStatus::OK => (OK.0, String::from(OK.1)),
            HTTPStatus::NotModified => (NOT_MODIFIED.0, String::from(NOT_MODIFIED.1)),
            HTTPStatus::NotFound => (NOT_FOUND.0, String::from(NOT_FOUND.1)),
            HTTPStatus::InternalServerError => (INTERNAL_SERVER_ERROR.0, String::from(INTERNAL_SERVER_ERROR.1))
        }
    }
}

const OK: (i16, &str) = (200, "OK");
const NOT_MODIFIED: (i16, &str) = (304, "Not Modified");
const NOT_FOUND: (i16, &str) = (404, "Not Found");
const INTERNAL_SERVER_ERROR: (i16, &str) = (500, "Internal Server Error");