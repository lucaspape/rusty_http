extern crate core;

use std::thread;
use crate::config::RustyHTTPConfig;
use crate::http::host::HTTPHost;
use crate::http::location::HTTPLocation;
use crate::http::server::HTTPServer;

mod http;
mod config;

const CONFIG_FILENAME: &str = "config.json";

fn main() {
    let c = RustyHTTPConfig::read(CONFIG_FILENAME);

    for s in c.servers {
        let mut default_host_locations: Vec<HTTPLocation> = Vec::new();

        for l in s.default_host.locations.iter() {
            default_host_locations.push(HTTPLocation::new(l.path.as_str(), l.root.as_str()));
        }

        let default_host = HTTPHost::new("default", default_host_locations);

        let mut hosts: Vec<HTTPHost> = Vec::new();

        for h in s.hosts.iter() {
            let mut host_locations: Vec<HTTPLocation> = Vec::new();

            for l in h.locations.iter() {
                host_locations.push(HTTPLocation::new(l.path.as_str(), l.root.as_str()));
            }

            hosts.push(HTTPHost::new(h.server_name.as_str(), host_locations));
        }

        let bind = s.bind.clone();

        thread::spawn(|| {
            HTTPServer::new(bind, default_host, hosts)
                .listen();
        });
    }

    loop {

    }
}