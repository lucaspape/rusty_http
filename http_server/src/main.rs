extern crate core;

use std::{fs, thread};
use std::collections::HashMap;
use std::time::Duration;
use libloading::{Library, Symbol};
use http_extension::Extension;
use crate::config::RustyHTTPConfig;
use crate::http::host::HTTPHost;
use crate::http::location::HTTPLocation;
use crate::http::server::HTTPServer;

mod http;
mod config;

const CONFIG_FILENAME: &str = "config.json";
const EXTENSIONS_DIR: &str = "../out/extensions/";

fn main() {
    for extension in fs::read_dir(EXTENSIONS_DIR).unwrap() {
        let path = extension.unwrap().path();
        load_extension(path.to_str().unwrap());
    }

    let c = RustyHTTPConfig::read(CONFIG_FILENAME);

    for s in c.servers {
        let mut default_host_locations: Vec<HTTPLocation> = Vec::new();

        for l in s.default_host.locations.iter() {
            let mut index = false;

            if l.index != None {
                index = l.index.unwrap()
            }

            default_host_locations.push(HTTPLocation::new(l.path.as_str(), l.root.as_str(), index));
        }

        let default_host = HTTPHost::new("default", default_host_locations);

        let mut hosts: Vec<HTTPHost> = Vec::new();

        for h in s.hosts.iter() {
            let mut host_locations: Vec<HTTPLocation> = Vec::new();

            for l in h.locations.iter() {
                let mut index = false;

                if l.index != None {
                    index = l.index.unwrap()
                }

                host_locations.push(HTTPLocation::new(l.path.as_str(), l.root.as_str(), index));
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
        thread::sleep(Duration::from_secs(60));
    }
}

fn load_extension(path: &str, config: HashMap<String, String>) -> Box<dyn Extension> {
    unsafe {
        type ExtensionCreate = unsafe fn() -> *mut dyn Extension;

        let lib = Library::new(path).unwrap();

        let constructor: Symbol<ExtensionCreate> = lib.get(b"_extension_create").unwrap();
        let boxed_raw = constructor();

        let mut extension = Box::from_raw(boxed_raw);
        extension.on_load(config);

        println!("Loaded extension {}", extension.name());

        return extension;
    }
}