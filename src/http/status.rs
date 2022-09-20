pub enum HTTPStatus {
    OK,
    NotFound,
    InternalServerError,
}

impl HTTPStatus {
    pub fn get(self) -> (i16, String) {
        return match self {
            HTTPStatus::OK => (OK.0, String::from(OK.1)),
            HTTPStatus::NotFound => (NOT_FOUND.0, String::from(NOT_FOUND.1)),
            HTTPStatus::InternalServerError => (INTERNAL_SERVER_ERROR.0, String::from(INTERNAL_SERVER_ERROR.1))
        }
    }
}

const OK: (i16, &str) = (200, "OK");
const NOT_FOUND: (i16, &str) = (404, "Not Found");
const INTERNAL_SERVER_ERROR: (i16, &str) = (500, "Internal Server Error");