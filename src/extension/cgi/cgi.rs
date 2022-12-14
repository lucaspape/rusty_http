use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::path::Path;
use std::process::{Command, Stdio};
use serde_json::Value;
use crate::common::mime::MimeType;
use crate::common::request::HTTPRequest;
use crate::common::status::HTTPStatus;
use crate::Extension;
use crate::extension::extension_handler::ExtensionHandler;

pub struct CGIExtension {
    pub target: String
}

impl Extension for CGIExtension {
    fn configure(&mut self, config: HashMap<String, Value>) {
        self.target = String::from(config.get("target").expect("No target in cgi extension").as_str().unwrap());
    }

    fn handler(&self) -> ExtensionHandler {
        return ExtensionHandler {
            request: Self::handle,
            args: Vec::from([self.target.clone()])
        };
    }

    fn name(&self) -> String {
        String::from("cgi")
    }
}

impl CGIExtension {
    fn handle(
        args: Vec<String>,
        location: &str,
        root: &str,
        index_files: Vec<String>,
        stream: &TcpStream,
        request: &HTTPRequest,
        body: &Vec<String>,
        write_header: fn(&TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> bool,
        write_bytes: fn(&TcpStream, Vec<u8>) -> bool
    ) -> bool {
        let mut request_path = String::from(&*request.path);

        if request_path.starts_with(location) {
            request_path = request_path.replacen(location, "", 1);
        }

        let mut file_path = String::from(root) + request_path.as_str();

        let path = Path::new(file_path.as_str());

        if !path.exists() {
            let msg = "No such file or directory";

            if !write_header(stream, HTTPStatus::NotFound, MimeType::Plain, msg.len(), None) {
                return false;
            }

            return write_bytes(stream, Vec::from(msg.as_bytes()));
        }

        if path.is_dir() {
            for index_file in index_files {
                let file_path_index = String::from(file_path.clone()) + "/" + index_file.as_str();
                let path = Path::new(file_path_index.as_str());

                if path.exists() && !path.is_dir() {
                    file_path = file_path_index;
                    break
                }
            }
        }

        let mut body_len: usize = 0;

        for l in body.iter() {
            body_len += l.len()
        }

        let mut cgi = Command::new("cgi-fcgi")
            .env("SCRIPT_FILENAME", file_path.clone())
            .env("SERVER_PROTOCOL", request.http_version.as_str())

            //TODO dont hardcode this
            .env("HTTP_X_FORWARDED_PROTO", "https")

            .env("QUERY_STRING", request.query.as_str())
            .env("REQUEST_URI", request.path.as_str())
            .env("REQUEST_METHOD", request.method.get_string())
            .env("CONTENT_LENGTH", format!("{}", body_len))
            .env("CONTENT_TYPE", request.content_type.clone().get())
            .env("SERVER_NAME", request.host.clone())
            .env("HTTP_USER_AGENT", request.user_agent.clone())
            .env("HTTP_HOST", request.host.clone())
            .env("HTTP_COOKIE", request.cookie.clone())
            .env("SCRIPT_NAME", request.path.as_str())
            .arg("-bind")
            .arg("-connect")
            .arg(args[0].clone())
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if body_len > 0 {
            let i = cgi.stdin.as_mut().unwrap();

            for l in body.iter() {
                i.write_all(l.as_bytes()).unwrap();
            }
        }

        let out = cgi.wait_with_output().unwrap();
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