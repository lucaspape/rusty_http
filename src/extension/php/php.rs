use std::net::TcpStream;
use std::process::Command;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::common::status::HTTPStatus;
use crate::Extension;
use crate::extension::extension_handler::ExtensionHandler;

pub struct PHPExtension {

}

impl Extension for PHPExtension {
    fn handler(&self) -> ExtensionHandler {
        return ExtensionHandler{request: Self::handle}
    }

    fn name(&self) -> String {
        String::from("php")
    }
}

impl PHPExtension {
    fn handle(
        location: &str,
        root: &str,
        index: bool,
        stream: &TcpStream,
        request: &HTTPRequest,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool {
        let mut request_path = String::from(&*request.path);

        if request_path.starts_with(location) {
            request_path = request_path.replacen(location, "", 1);
        }

        let file_path = String::from(root) + request_path.as_str();

        let mut cgi = Command::new("cgi-fcgi");
        cgi.env("SCRIPT_FILENAME", file_path);
        cgi.env("QUERY_STRING", "");
        cgi.env("REQUEST_URI", request.path.as_str());
        cgi.env("REQUEST_METHOD", request.method.get_string());

        cgi.arg("-bind");
        cgi.arg("-connect");
        cgi.arg("localhost:9000");

        let out = cgi.output().unwrap();

        let st = String::from_utf8_lossy(&out.stdout);

        let mut status = HTTPStatus::OK;
        let mut content_type = MimeType::Html;
        let mut additional_headers: Vec<String> = Vec::new();

        let mut header_done = false;
        let mut content = String::from("");

        for line in st.lines() {
            if header_done {
                content += line;
                content += "\n"
            }else{
                let mut is_additional = true;

                if line.is_empty() {
                    header_done = true;
                    continue;
                }

                let s_type = HTTPRequest::parse_header("Content-Type: ", &String::from(line));
                if s_type != None {
                    content_type = MimeType::parse(s_type.unwrap().as_str()).unwrap();
                    is_additional = false;
                }

                let s_status = HTTPRequest::parse_header("Status: ", &String::from(line));
                if s_status != None {
                    status = HTTPStatus::parse(s_status.unwrap().as_str()).unwrap();
                    is_additional = false;
                }

                if is_additional {
                    additional_headers.push(String::from(line));
                }
            }
        }

        if !write_header(stream, status, content_type, content.len(), Some(additional_headers)) {
            return false
        }

        return write_bytes(stream, Vec::from(content.as_bytes()));
    }
}