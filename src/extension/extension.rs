use crate::extension::extension_handler::ExtensionHandler;

pub trait Extension {
    fn handler(&self) -> ExtensionHandler;
}