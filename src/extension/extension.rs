use std::collections::HashMap;
use crate::extension::extension_handler::ExtensionHandler;

pub trait Extension {
    fn configure(&mut self, config: HashMap<String, serde_json::Value>);
    fn handler(&self) -> ExtensionHandler;
    fn name(&self) -> String;
}