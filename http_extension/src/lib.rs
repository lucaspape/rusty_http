pub mod extension_handler;

use std::any::Any;
use std::collections::HashMap;
use crate::extension_handler::ExtensionHandler;

pub struct Extension {
    pub name: fn (object: &mut dyn Any) -> String,
    pub on_load: fn(object: &mut dyn Any, config: HashMap<String, String>) -> bool,
    pub handler: fn(object: &mut dyn Any) -> ExtensionHandler,
}

pub struct ExtensionWrapper {
    pub extension: Box<Extension>,
    pub object: Box<dyn Any>
}