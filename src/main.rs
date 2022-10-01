extern crate core;

use std::thread;
use std::time::Duration;
use crate::config::{HostConfig, RustyHTTPConfig};
use crate::extension::extension::Extension;
use crate::extension::file_extension::FileExtension;
use crate::http::host::HTTPHost;
use crate::http::location::HTTPLocation;
use crate::http::server::HTTPServer;

mod http;
mod config;
mod extension;
mod common;

const CONFIG_FILENAME: &str = "config.json";

fn create_host(config: &HostConfig) -> HTTPHost {
    let mut host_locations: Vec<HTTPLocation> = Vec::new();

    for l in config.locations.iter() {
        let mut index = false;

        if l.index != None {
            index = l.index.unwrap()
        }

        host_locations.push(HTTPLocation::new(l.path.as_str(), l.root.as_str(), index, FileExtension{}.handler()))
    }

    return HTTPHost::new(config.server_name.as_str(), host_locations);
}

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