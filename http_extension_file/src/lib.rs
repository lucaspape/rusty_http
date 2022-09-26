use http_extension::{declare_extension, Extension};

#[derive(Debug, Default)]
pub struct FileExtension {

}

impl Extension for FileExtension {
    fn name(&self) -> &'static str {
        "File Extension"
    }

    fn on_load(&self) {
        println!("init file");
    }
}

declare_extension!(FileExtension, FileExtension::default);