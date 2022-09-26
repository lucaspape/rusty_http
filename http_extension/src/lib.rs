pub trait Extension {
    fn name(&self) -> &'static str;
    fn on_load(&self);
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