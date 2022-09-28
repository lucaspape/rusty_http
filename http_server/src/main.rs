extern crate core;

use std::{fs, thread};
use std::collections::HashMap;
use std::time::Duration;
use libloading::{Library, Symbol};
use http_extension::Extension;
use crate::config::{HostConfig, RustyHTTPConfig};
use crate::http::host::HTTPHost;
use crate::http::location::HTTPLocation;
use crate::http::server::HTTPServer;

mod http;
mod config;

const CONFIG_FILENAME: &str = "config.json";
const EXTENSIONS_DIR: &str = "extensions/";
const DEFAULT_EXTENSION: &str = "http_extension_file";

fn create_host(config: &HostConfig, extensions: &HashMap<String, String>) -> HTTPHost {
    let mut host_locations: Vec<HTTPLocation> = Vec::new();

    for l in config.locations.iter() {
        let mut extension_path = extensions.get(DEFAULT_EXTENSION);

        if extension_path == None {
            panic!("Could not find default extension {}", DEFAULT_EXTENSION);
        }

        let extension_name = l.extension.as_ref();

        if l.extension != None {
            extension_path = extensions.get(extension_name.unwrap().as_str());
        }

        if extension_path == None {
            panic!("Could not find extension {}", extension_name.unwrap().as_str());
        }

        println!("Loading extension {}...", extension_path.unwrap());

        let mut extension = load_extension(extension_path.unwrap());

        let mut config: HashMap<String, String> = HashMap::new();
        config.extend(l.config.clone());
        config.insert(String::from("path"), l.path.to_string());

        if !extension.on_load(config) {
            panic!("Failed loading extension {}", extension_path.unwrap());
        }

        let location = HTTPLocation::new(l.path.as_str(), extension.handler());
        host_locations.push(location)
    }

    return HTTPHost::new(config.server_name.as_str(), host_locations);
}

fn main() {
    let extensions = find_extensions(EXTENSIONS_DIR);

    let c = RustyHTTPConfig::read(CONFIG_FILENAME);

    for s in c.servers {
        let default_host = create_host(&s.default_host, &extensions);

        let mut hosts: Vec<HTTPHost> = Vec::new();

        for h in s.hosts.iter() {
            let host = create_host(h, &extensions);
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

fn load_extension(path: &str) -> Box<dyn Extension> {
    unsafe {
        type ExtensionCreate = unsafe fn() -> *mut (dyn Extension);

        let lib = Library::new(path).unwrap();

        let constructor: Symbol<ExtensionCreate> = lib.get(b"_extension_create").unwrap();
        let boxed_raw = constructor();

        let extension = Box::from_raw(boxed_raw);

        return extension;
    }
}

fn find_extensions(path: &str) -> HashMap<String, String> {
    let mut extensions = HashMap::new();

    for e in fs::read_dir(path).unwrap() {
        let path = e.unwrap().path();
        let mut extension = load_extension(path.to_str().unwrap());

        extensions.insert(String::from(extension.name().clone()), String::from(path.to_str().unwrap().clone()));
    }

    return extensions;
}