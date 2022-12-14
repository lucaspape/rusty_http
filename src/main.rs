extern crate core;

use std::thread;
use std::time::Duration;
use config::{HostConfig, RustyHTTPConfig};
use extension::extension::Extension;
use extension::file::file::FileExtension;
use extension::cgi::cgi::CGIExtension;
use http::host::HTTPHost;
use http::location::HTTPLocation;
use http::server::HTTPServer;

mod http;
mod config;
mod extension;
mod common;

const CONFIG_FILENAME: &str = "config.json";

fn main() {
    let c = RustyHTTPConfig::read(CONFIG_FILENAME);

    for s in c.servers {
        let default_host = create_host(&s.default_host);

        let mut hosts: Vec<HTTPHost> = Vec::new();

        for h in s.hosts.iter() {
            let host = create_host(h);
            hosts.push(host);
        }

        let bind = s.bind.clone();

        thread::spawn(|| {
            HTTPServer::new(bind, default_host, hosts)
                .listen();
        });
    }

    loop {
        thread::sleep(Duration::from_secs(60));
    }
}

fn create_host(config: &HostConfig) -> HTTPHost {
    let mut host_locations: Vec<HTTPLocation> = Vec::new();

    for l in config.locations.iter() {
        let mut extension_name = "file";

        if l.extension != None {
            extension_name = l.extension.as_ref().unwrap().as_str();
        }

        let mut extension = get_extension(extension_name);
        extension.configure(l.config.clone().unwrap());

        let mut index_files = l.index_files.clone();

        if let None = index_files {
            index_files = Some(Vec::new());
        }

        host_locations.push(HTTPLocation::new(l.path.as_str(), l.root.as_str(), extension.handler(), index_files.unwrap()))
    }

    return HTTPHost::new(config.server_name.as_str(), host_locations);
}

fn get_extension(name: &str) -> Box<dyn Extension> {
    return match name {
        "file" => {
            Box::new(FileExtension{
                index: false
            })
        },
        "cgi" => {
            Box::new(CGIExtension{
                target: "".to_string()
            })
        },
        _ => {
            panic!("Could not find extension {}", name)
        }
    }
}