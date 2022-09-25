use std::fs;
use chrono::{DateTime, Utc};
use humansize::{DECIMAL, format_size};
use crate::http::html::HTML;

pub fn generate_index(local_path: &str, path: &str) -> String {
    let paths = fs::read_dir(local_path).unwrap();

    let mut table_entries = String::from("");

    table_entries += generate_index_entry("../", "", "").as_str();

    for index_entry in paths {
        let index_entry = index_entry.unwrap().path();
        let mut name = String::from(index_entry.file_stem().unwrap().to_str().unwrap());

        if index_entry.is_file() {
            let ext = index_entry.extension();

            if ext != None {
                name += ".";
                name += ext.unwrap().to_str().unwrap();
            }
        }

        let metadata = index_entry.metadata().unwrap();
        let modified: DateTime<Utc> = metadata.modified().unwrap().into();

        let mut size = String::from("-");

        if index_entry.is_file() {
            size = format_size(metadata.len(), DECIMAL);
        }

        table_entries += generate_index_entry(name.as_str(), modified.format("%Y-%m-%d %T").to_string().as_str(), size.as_str()).as_str();
    }

    let html = HTML::html(
        HTML::body(
            (
                HTML::h1(format!("Index of {path}").as_str()) +
                HTML::hr("").as_str() +
                HTML::table(table_entries.as_str()).as_str() +
                HTML::hr("").as_str()
            ).as_str()
        ).as_str()
    );

    return html;
}

pub fn generate_index_entry(name: &str, modified: &str, size: &str) -> String {
    return HTML::tr(
        (
            HTML::td(HTML::a(name, name).as_str()) +
            HTML::td(modified).as_str() +
            HTML::td(size).as_str()).as_str()
    )
}