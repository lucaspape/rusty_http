pub enum HTTPStatus {
    Continue,
    SwitchingProtocols,
    Processing,
    EarlyHints,

    OK,
    Created,
    Accepted,
    NonAuthoritativeInformation,
    NoContent,
    ResetContent,
    PartialContent,
    MultiStatus,
    AlreadyReported,
    IMUsed,

    MultipleChoice,
    MovedPermanently,
    Found,
    SeeOther,
    NotModified,
    TemporaryRedirect,
    PermanentRedirect,

    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    NotAcceptable,
    ProxyAuthenticationRequired,
    RequestTimeout,
    Conflict,
    Gone,
    LengthRequired,
    PreconditionFailed,
    PayloadTooLarge,
    URITooLong,
    UnsupportedMediaType,
    RangeNotSatisfiable,
    ExpectationFailed,
    ImATeapot,
    MisdirectedRequest,
    UnprocessableEntity,
    Locked,
    FailedDependency,
    TooEarly,
    UpgradeRequired,
    PreconditionRequired,
    TooManyRequests,
    RequestHeaderFieldsTooLarge,
    UnavailableForLegalReasons,

    InternalServerError,
    NotImplemented,
    BadGateway,
    ServiceUnavailable,
    GatewayTimeout,
    HTTPVersionNotSupported,
    VariantAlsoNegotiates,
    InsufficientStorage,
    LoopDetected,
    NotExtended,
    NetworkAuthenticationRequired
}

impl HTTPStatus {
    pub fn get(self) -> (i16, String) {
        return match self {
            HTTPStatus::Continue => (CONTINUE.0, String::from(CONTINUE.1)),
            HTTPStatus::SwitchingProtocols => (SWITCHING_PROTOCOLS.0, String::from(SWITCHING_PROTOCOLS.1)),
            HTTPStatus::Processing => (PROCESSING.0, String::from(PROCESSING.1)),
            HTTPStatus::EarlyHints => (EARLY_HINTS.0, String::from(EARLY_HINTS.1)),
            HTTPStatus::OK => (OK.0, String::from(OK.1)),
            HTTPStatus::Created => (CREATED.0, String::from(CREATED.1)),
            HTTPStatus::Accepted => (ACCEPTED.0, String::from(ACCEPTED.1)),
            HTTPStatus::NonAuthoritativeInformation => (NON_AUTHORITATIVE_INFORMATION.0, String::from(NON_AUTHORITATIVE_INFORMATION.1)),
            HTTPStatus::NoContent => (NO_CONTENT.0, String::from(NO_CONTENT.1)),
            HTTPStatus::ResetContent => (RESET_CONTENT.0, String::from(RESET_CONTENT.1)),
            HTTPStatus::PartialContent => (PARTIAL_CONTENT.0, String::from(PARTIAL_CONTENT.1)),
            HTTPStatus::MultiStatus => (MULTI_STATUS.0, String::from(MULTI_STATUS.1)),
            HTTPStatus::AlreadyReported => (ALREADY_REPORTED.0, String::from(ALREADY_REPORTED.1)),
            HTTPStatus::IMUsed => (IM_USED.0, String::from(IM_USED.1)),
            HTTPStatus::MultipleChoice => (MULTIPLE_CHOICE.0, String::from(MULTIPLE_CHOICE.1)),
            HTTPStatus::MovedPermanently => (MOVED_PERMANENTLY.0, String::from(MOVED_PERMANENTLY.1)),
            HTTPStatus::Found => (FOUND.0, String::from(FOUND.1)),
            HTTPStatus::SeeOther => (SEE_OTHER.0, String::from(SEE_OTHER.1)),
            HTTPStatus::NotModified => (NOT_MODIFIED.0, String::from(NOT_MODIFIED.1)),
            HTTPStatus::TemporaryRedirect => (TEMPORARY_REDIRECT.0, String::from(TEMPORARY_REDIRECT.1)),
            HTTPStatus::PermanentRedirect => (PERMANENT_REDIRECT.0, String::from(PERMANENT_REDIRECT.1)),
            HTTPStatus::BadRequest => (BAD_REQUEST.0, String::from(BAD_REQUEST.1)),
            HTTPStatus::Unauthorized => (UNAUTHORIZED.0, String::from(UNAUTHORIZED.1)),
            HTTPStatus::PaymentRequired => (PAYMENT_REQUIRED.0, String::from(PAYMENT_REQUIRED.1)),
            HTTPStatus::Forbidden => (FORBIDDEN.0, String::from(FORBIDDEN.1)),
            HTTPStatus::NotFound => (NOT_FOUND.0, String::from(NOT_FOUND.1)),
            HTTPStatus::MethodNotAllowed => (METHOD_NOT_ALLOWED.0, String::from(METHOD_NOT_ALLOWED.1)),
            HTTPStatus::NotAcceptable => (NOT_ACCEPTABLE.0, String::from(NOT_ACCEPTABLE.1)),
            HTTPStatus::ProxyAuthenticationRequired => (PROXY_AUTHENTICATION_REQUIRED.0, String::from(PROXY_AUTHENTICATION_REQUIRED.1)),
            HTTPStatus::RequestTimeout => (REQUEST_TIMEOUT.0, String::from(REQUEST_TIMEOUT.1)),
            HTTPStatus::Conflict => (CONFLICT.0, String::from(CONFLICT.1)),
            HTTPStatus::Gone => (GONE.0, String::from(GONE.1)),
            HTTPStatus::LengthRequired => (LENGTH_REQUIRED.0, String::from(LENGTH_REQUIRED.1)),
            HTTPStatus::PreconditionFailed => (PRECONDITION_FAILED.0, String::from(PRECONDITION_FAILED.1)),
            HTTPStatus::PayloadTooLarge => (PAYLOAD_TOO_LARGE.0, String::from(PAYLOAD_TOO_LARGE.1)),
            HTTPStatus::URITooLong => (URI_TOO_LONG.0, String::from(URI_TOO_LONG.1)),
            HTTPStatus::UnsupportedMediaType => (UNSUPPORTED_MEDIA_TYPE.0, String::from(UNSUPPORTED_MEDIA_TYPE.1)),
            HTTPStatus::RangeNotSatisfiable => (RANGE_NOT_SATISFIABLE.0, String::from(RANGE_NOT_SATISFIABLE.1)),
            HTTPStatus::ExpectationFailed => (EXPECTATION_FAILED.0, String::from(EXPECTATION_FAILED.1)),
            HTTPStatus::ImATeapot => (IM_A_TEAPOT.0, String::from(IM_A_TEAPOT.1)),
            HTTPStatus::MisdirectedRequest => (MISDIRECTED_REQUEST.0, String::from(MISDIRECTED_REQUEST.1)),
            HTTPStatus::UnprocessableEntity => (UNPROCESSABLE_ENTITY.0, String::from(UNPROCESSABLE_ENTITY.1)),
            HTTPStatus::Locked => (LOCKED.0, String::from(LOCKED.1)),
            HTTPStatus::FailedDependency => (FAILED_DEPENDENCY.0, String::from(FAILED_DEPENDENCY.1)),
            HTTPStatus::TooEarly => (TOO_EARLY.0, String::from(TOO_EARLY.1)),
            HTTPStatus::UpgradeRequired => (UPGRADE_REQUIRED.0, String::from(UPGRADE_REQUIRED.1)),
            HTTPStatus::PreconditionRequired => (PRECONDITION_REQUIRED.0, String::from(PRECONDITION_REQUIRED.1)),
            HTTPStatus::TooManyRequests => (TOO_MANY_REQUESTS.0, String::from(TOO_MANY_REQUESTS.1)),
            HTTPStatus::RequestHeaderFieldsTooLarge => (REQUEST_HEADER_FIELDS_TOO_LARGE.0, String::from(REQUEST_HEADER_FIELDS_TOO_LARGE.1)),
            HTTPStatus::UnavailableForLegalReasons => (UNAVAILABLE_FOR_LEGAL_REASONS.0, String::from(UNAVAILABLE_FOR_LEGAL_REASONS.1)),
            HTTPStatus::InternalServerError => (INTERNAL_SERVER_ERROR.0, String::from(INTERNAL_SERVER_ERROR.1)),
            HTTPStatus::NotImplemented => (NOT_IMPLEMENTED.0, String::from(NOT_IMPLEMENTED.1)),
            HTTPStatus::BadGateway => (BAD_GATEWAY.0, String::from(BAD_GATEWAY.1)),
            HTTPStatus::ServiceUnavailable => (SERVICE_UNAVAILABLE.0, String::from(SERVICE_UNAVAILABLE.1)),
            HTTPStatus::GatewayTimeout => (GATEWAY_TIMEOUT.0, String::from(GATEWAY_TIMEOUT.1)),
            HTTPStatus::HTTPVersionNotSupported => (HTTP_VERSION_NOT_SUPPORTED.0, String::from(HTTP_VERSION_NOT_SUPPORTED.1)),
            HTTPStatus::VariantAlsoNegotiates => (VARIANT_ALSO_NEGOTIATES.0, String::from(VARIANT_ALSO_NEGOTIATES.1)),
            HTTPStatus::InsufficientStorage => (INSUFFICIENT_STORAGE.0, String::from(INSUFFICIENT_STORAGE.1)),
            HTTPStatus::LoopDetected => (LOOP_DETECTED.0, String::from(LOOP_DETECTED.1)),
            HTTPStatus::NotExtended => (NOT_EXTENDED.0, String::from(NOT_EXTENDED.1)),
            HTTPStatus::NetworkAuthenticationRequired => (NETWORK_AUTHENTICATION_REQUIRED.0, String::from(NETWORK_AUTHENTICATION_REQUIRED.1)),
        }
    }

    pub fn parse(s: &str) -> Option<HTTPStatus> {
        if s.contains("100") {
            Some(HTTPStatus::Continue)
        }else if s.contains("101") {
            Some(HTTPStatus::SwitchingProtocols)
        }else if s.contains("102") {
            Some(HTTPStatus::Processing)
        }else if s.contains("103") {
            Some(HTTPStatus::EarlyHints)
        }else if s.contains("200") {
            Some(HTTPStatus::OK)
        }else if s.contains("201") {
            Some(HTTPStatus::Created)
        }else if s.contains("202") {
            Some(HTTPStatus::Accepted)
        }else if s.contains("203") {
            Some(HTTPStatus::NonAuthoritativeInformation)
        }else if s.contains("204") {
            Some(HTTPStatus::NoContent)
        }else if s.contains("205") {
            Some(HTTPStatus::ResetContent)
        }else if s.contains("206") {
            Some(HTTPStatus::PartialContent)
        }else if s.contains("207") {
            Some(HTTPStatus::MultiStatus)
        }else if s.contains("208") {
            Some(HTTPStatus::AlreadyReported)
        }else if s.contains("226") {
            Some(HTTPStatus::IMUsed)
        }else if s.contains("300") {
            Some(HTTPStatus::MultipleChoice)
        }else if s.contains("301") {
            Some(HTTPStatus::MovedPermanently)
        }else if s.contains("302") {
            Some(HTTPStatus::Found)
        }else if s.contains("303") {
            Some(HTTPStatus::SeeOther)
        }else if s.contains("304") {
            Some(HTTPStatus::NotModified)
        }else if s.contains("307") {
            Some(HTTPStatus::TemporaryRedirect)
        }else if s.contains("308") {
            Some(HTTPStatus::PermanentRedirect)
        }else if s.contains("400") {
            Some(HTTPStatus::BadRequest)
        }else if s.contains("401") {
            Some(HTTPStatus::Unauthorized)
        }else if s.contains("402") {
            Some(HTTPStatus::PaymentRequired)
        }else if s.contains("403") {
            Some(HTTPStatus::Forbidden)
        }else if s.contains("404") {
            Some(HTTPStatus::NotFound)
        }else if s.contains("405") {
            Some(HTTPStatus::MethodNotAllowed)
        }else if s.contains("406") {
            Some(HTTPStatus::NotAcceptable)
        }else if s.contains("407") {
            Some(HTTPStatus::ProxyAuthenticationRequired)
        }else if s.contains("408") {
            Some(HTTPStatus::RequestTimeout)
        }else if s.contains("409") {
            Some(HTTPStatus::Conflict)
        }else if s.contains("410") {
            Some(HTTPStatus::Gone)
        }else if s.contains("411") {
            Some(HTTPStatus::LengthRequired)
        }else if s.contains("412") {
            Some(HTTPStatus::PreconditionFailed)
        }else if s.contains("413") {
            Some(HTTPStatus::PayloadTooLarge)
        }else if s.contains("414") {
            Some(HTTPStatus::URITooLong)
        }else if s.contains("415") {
            Some(HTTPStatus::UnsupportedMediaType)
        }else if s.contains("416") {
            Some(HTTPStatus::RangeNotSatisfiable)
        }else if s.contains("417") {
            Some(HTTPStatus::ExpectationFailed)
        }else if s.contains("418") {
            Some(HTTPStatus::ImATeapot)
        }else if s.contains("421") {
            Some(HTTPStatus::MisdirectedRequest)
        }else if s.contains("422") {
            Some(HTTPStatus::UnprocessableEntity)
        }else if s.contains("423") {
            Some(HTTPStatus::Locked)
        }else if s.contains("424") {
            Some(HTTPStatus::FailedDependency)
        }else if s.contains("425") {
            Some(HTTPStatus::TooEarly)
        }else if s.contains("426") {
            Some(HTTPStatus::UpgradeRequired)
        }else if s.contains("428") {
            Some(HTTPStatus::PreconditionRequired)
        }else if s.contains("429") {
            Some(HTTPStatus::TooManyRequests)
        }else if s.contains("431") {
            Some(HTTPStatus::RequestHeaderFieldsTooLarge)
        }else if s.contains("451") {
            Some(HTTPStatus::UnavailableForLegalReasons)
        }else if s.contains("500") {
            Some(HTTPStatus::InternalServerError)
        }else if s.contains("501") {
            Some(HTTPStatus::NotImplemented)
        }else if s.contains("502") {
            Some(HTTPStatus::BadGateway)
        }else if s.contains("503") {
            Some(HTTPStatus::ServiceUnavailable)
        }else if s.contains("504") {
            Some(HTTPStatus::GatewayTimeout)
        }else if s.contains("505") {
            Some(HTTPStatus::HTTPVersionNotSupported)
        }else if s.contains("506") {
            Some(HTTPStatus::VariantAlsoNegotiates)
        }else if s.contains("507") {
            Some(HTTPStatus::InsufficientStorage)
        }else if s.contains("508") {
            Some(HTTPStatus::LoopDetected)
        }else if s.contains("510") {
            Some(HTTPStatus::NotExtended)
        }else if s.contains("511") {
            Some(HTTPStatus::NetworkAuthenticationRequired)
        }else{
            None
        }
    }
}

const CONTINUE: (i16, &str) = (100, "Continue");
const SWITCHING_PROTOCOLS: (i16, &str) = (101, "Switching Protocols");
const PROCESSING: (i16, &str) = (102, "Processing");
const EARLY_HINTS: (i16, &str) = (103, "Early Hints");

const OK: (i16, &str) = (200, "OK");
const CREATED: (i16, &str) = (201, "Created");
const ACCEPTED: (i16, &str) = (202, "Accepted");
const NON_AUTHORITATIVE_INFORMATION: (i16, &str) = (203, "Non-Authoritative Information");
const NO_CONTENT: (i16, &str) = (204, "No Content");
const RESET_CONTENT: (i16, &str) = (205, "Reset Content");
const PARTIAL_CONTENT: (i16, &str) = (206, "Partial Content");
const MULTI_STATUS: (i16, &str) = (207, "Multi-Status");
const ALREADY_REPORTED: (i16, &str) = (208, "Already Reported");
const IM_USED: (i16, &str) = (226, "IM Used");

const MULTIPLE_CHOICE: (i16, &str) = (300, "Multiple Choices");
const MOVED_PERMANENTLY: (i16, &str) = (301, "Moved Permanently");
const FOUND: (i16, &str) = (302, "Found");
const SEE_OTHER: (i16, &str) = (303, "See Other");
const NOT_MODIFIED: (i16, &str) = (304, "Not Modified");
const TEMPORARY_REDIRECT: (i16, &str) = (307, "Temporary Redirect");
const PERMANENT_REDIRECT: (i16, &str) = (308, "Permanent Redirect");

const BAD_REQUEST: (i16, &str) = (400, "Bad Request");
const UNAUTHORIZED: (i16, &str) = (401, "Unauthorized");
const PAYMENT_REQUIRED: (i16, &str) = (402, "Payment Required");
const FORBIDDEN: (i16, &str) = (403, "Forbidden");
const NOT_FOUND: (i16, &str) = (404, "Not Found");
const METHOD_NOT_ALLOWED: (i16, &str) = (405, "Method Not Allowed");
const NOT_ACCEPTABLE: (i16, &str) = (406, "Not Acceptable");
const PROXY_AUTHENTICATION_REQUIRED: (i16, &str) = (407, "Proxy Authentication Required");
const REQUEST_TIMEOUT: (i16, &str) = (408, "Request Timeout");
const CONFLICT: (i16, &str) = (409, "Conflict");
const GONE: (i16, &str) = (410, "Gone");
const LENGTH_REQUIRED: (i16, &str) = (411, "Length Required");
const PRECONDITION_FAILED: (i16, &str) = (412, "Precondition Failed");
const PAYLOAD_TOO_LARGE: (i16, &str) = (413, "Payload Too Large");
const URI_TOO_LONG: (i16, &str) = (414, "URI Too Long");
const UNSUPPORTED_MEDIA_TYPE: (i16, &str) = (415, "Unsupported Media Type");
const RANGE_NOT_SATISFIABLE: (i16, &str) = (416, "Range Not Satisfiable");
const EXPECTATION_FAILED: (i16, &str) = (417, "Expectation Failed");
const IM_A_TEAPOT: (i16, &str) = (418, "I'm a teapot");
const MISDIRECTED_REQUEST: (i16, &str) = (421, "Misdirected Request");
const UNPROCESSABLE_ENTITY: (i16, &str) = (422, "Unprocessable Entity");
const LOCKED: (i16, &str) = (423, "Locked");
const FAILED_DEPENDENCY: (i16, &str) = (424, "Failed Dependency");
const TOO_EARLY: (i16, &str) = (425, "Too Early");
const UPGRADE_REQUIRED: (i16, &str) = (426, "Upgrade Required");
const PRECONDITION_REQUIRED: (i16, &str) = (428, "Precondition Required");
const TOO_MANY_REQUESTS: (i16, &str) = (429, "Too Many Requests");
const REQUEST_HEADER_FIELDS_TOO_LARGE: (i16, &str) = (431, "Request Header Fields Too Large");
const UNAVAILABLE_FOR_LEGAL_REASONS: (i16, &str) = (451, "Unavailable For Legal Reasons");


const INTERNAL_SERVER_ERROR: (i16, &str) = (500, "Internal Server Error");
const NOT_IMPLEMENTED: (i16, &str) = (501, "Not Implemented");
const BAD_GATEWAY: (i16, &str) = (502, "Bad Gateway");
const SERVICE_UNAVAILABLE: (i16, &str) = (503, "Service Unavailable");
const GATEWAY_TIMEOUT: (i16, &str) = (504, "Gateway Timeout");
const HTTP_VERSION_NOT_SUPPORTED: (i16, &str) = (505, "HTTP Version Not Supported");
const VARIANT_ALSO_NEGOTIATES: (i16, &str) = (506, "Variant Also Negotiates");
const INSUFFICIENT_STORAGE: (i16, &str) = (507, "Insufficient Storage");
const LOOP_DETECTED: (i16, &str) = (508, "Loop Detected");
const NOT_EXTENDED: (i16, &str) = (510, "Not Extended");
const NETWORK_AUTHENTICATION_REQUIRED: (i16, &str) = (511, "Network Authentication Required");