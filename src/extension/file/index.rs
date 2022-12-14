use std::fs;
use chrono::{DateTime, Utc};
use humansize::{DECIMAL, format_size};
use crate::common::html::HTML;

pub fn generate_index(local_path: &str, path: &str) -> String {
    let paths = fs::read_dir(local_path).unwrap();

    let mut table_entries = String::from("");

    table_entries += HTML::tr(
        (
            HTML::th("Path") +
            HTML::th("Modified").as_str() +
            HTML::th("Size").as_str()
        ).as_str()
    ).as_str();

    table_entries += generate_index_entry("../", String::from(""), "","", false).as_str();

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

        table_entries += generate_index_entry(name.as_str(), String::from(path),modified.format("%Y-%m-%d %T").to_string().as_str(), size.as_str(), index_entry.is_file()).as_str();
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

pub fn generate_index_entry(name: &str, mut path: String, modified: &str, size: &str, is_file: bool) -> String {
    if !path.ends_with("/") {
        path += "/"
    }

    if name != "../" {
        path += name;

        if !is_file {
            path += "/";
        }
    } else {
        path = String::from(name);
    }

    return HTML::tr(
        (
            HTML::td(HTML::a(name, path.as_str()).as_str()) +
            HTML::td(modified).as_str() +
            HTML::td(size).as_str()).as_str()
    )
}