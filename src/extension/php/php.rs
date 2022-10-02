use std::collections::HashMap;
use std::net::TcpStream;
use std::process::Command;
use serde_json::Value;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::common::status::HTTPStatus;
use crate::Extension;
use crate::extension::extension_handler::ExtensionHandler;

pub struct PHPExtension {
    pub root: String,
    pub target: String
}

impl Extension for PHPExtension {
    fn configure(&mut self, config: HashMap<String, Value>) {
        self.root = String::from(config.get("root").expect("No root in php extension").as_str().unwrap());
        self.target = String::from(config.get("target").expect("No target in php extension").as_str().unwrap());
    }

    fn handler(&self) -> ExtensionHandler {
        return ExtensionHandler {
            request: Self::handle,
            args: Vec::from([self.root.clone(), self.target.clone()])
        };
    }

    fn name(&self) -> String {
        String::from("php")
    }
}

impl PHPExtension {
    fn handle(
        args: Vec<String>,
        location: &str,
        stream: &TcpStream,
        request: &HTTPRequest,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool,
    ) -> bool {
        let mut request_path = String::from(&*request.path);

        if request_path.starts_with(location) {
            request_path = request_path.replacen(location, "", 1);
        }

        let file_path = args[0].clone() + request_path.as_str();

        let mut cgi = Command::new("cgi-fcgi");
        cgi.env("SCRIPT_FILENAME", file_path);
        cgi.env("QUERY_STRING", "");
        cgi.env("REQUEST_URI", request.path.as_str());
        cgi.env("REQUEST_METHOD", request.method.get_string());

        cgi.arg("-bind");
        cgi.arg("-connect");
        cgi.arg(args[1].clone());

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
            } else {
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
            return false;
        }

        return write_bytes(stream, Vec::from(content.as_bytes()));
    }
}