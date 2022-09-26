use std::collections::HashMap;
use std::net::TcpStream;
use http_common::mime::MimeType;
use http_common::request::HTTPRequest;
use http_common::status::HTTPStatus;

pub trait Extension {
    fn name(&mut self) -> &'static str;
    fn on_load(&mut self, config: HashMap<String, String>);
    fn handle_request(&mut self,
                      stream: Option<TcpStream>,
                      request: &HTTPRequest,
                      write_header: fn(TcpStream, HTTPStatus, MimeType, usize, Option<Vec<String>>) -> Option<TcpStream>,
                      write_bytes: fn(TcpStream, Vec<u8>) -> Option<TcpStream>
    ) -> Option<TcpStream>;
}

#[macro_export]
macro_rules! declare_extension {
    ($extension_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _extension_create() -> *mut dyn $crate::Extension {
            let constructor: fn() -> $extension_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::Extension> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}
