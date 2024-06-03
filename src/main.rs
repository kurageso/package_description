use serde_json;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Ok(val) = read_dir(&args[1]) {
        for (v, _s) in val.into_iter() {
            let package_json_path = std::format!("{}/{}", v.display(), "package.json");


            if let Ok(content) = read_file(package_json_path) {
                print_package_description(v.to_str().unwrap().to_string(), content)
            }
        }
    }
}

fn read_file(file_name: String) -> Result<String, std::io::Error> {
    let mut f = File::open(&file_name)?;
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    Ok(content)
}

fn print_package_description(dir_name: String, content: String) {
    let result: serde_json::Result<serde_json::Value> = serde_json::from_str(&content);

    if let Ok(value) = result {
        println!(
            "{}\n\t- name: {}\n\t- description:{}",
            dir_name,
            value["name"], value["description"]
        )
    }
}

fn read_dir(path: &str) -> Result<Vec<(path::PathBuf, String)>, std::io::Error> {
    let dir = fs::read_dir(path)?;

    let mut files: Vec<(path::PathBuf, String)> = Vec::new();
    for item in dir.filter_map(|r| r.ok()) {
        match item.file_name().into_string() {
            Ok(s) => {
              files.push((item.path(), s));
            }
            _ => (),
        }
    }

    Ok(files)
}
