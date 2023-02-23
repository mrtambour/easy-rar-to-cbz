use std::{env, fs};

use tempfile::TempDir;
use unrar::Archive;
use zip_extensions::*;

fn get_current_directory() -> String {
    env::current_dir()
        .expect("error getting current directory")
        .into_os_string()
        .into_string()
        .expect("error getting path")
}

fn scan_for_rar(current_dir: &String) -> Vec<String> {
    let mut archives_list = Vec::new();
    for entry in fs::read_dir(current_dir).expect("error occurred while trying to scan directory") {
        let file_name = entry
            .expect("error while scanning directory")
            .file_name()
            .into_string()
            .expect("error getting file name");

        if file_name.ends_with(".rar") || file_name.ends_with(".cbr") {
            archives_list.push(file_name);
        }
    }
    archives_list
}

fn process_archives(archive_list: Vec<String>) -> Option<String> {
    for archive in archive_list {
        let target_archive_name = format!("{}.zip", archive);
        let temp_dir = TempDir::new().expect("error creating temporary folder");
        let path_buf = temp_dir.into_path();
        let temp_path = path_buf
            .to_str()
            .expect("error getting path string")
            .to_string();

        println!("processing: {}", archive.clone());
        extract_rar(vec![archive], &temp_path);
        let path = std::path::Path::new(&target_archive_name);
        let file = fs::File::create(path).expect("error creating new file");
        zip_create_from_directory(&path.to_path_buf(), &path_buf)
            .expect("error zipping files from directory");
        fs::remove_dir_all(&temp_path).expect("error deleting temporary folder");
    }
    None
}

fn extract_rar(archives: Vec<String>, target_directory: &String) {
    for rar in archives {
        Archive::new(rar)
            .extract_to(target_directory.to_string())
            .expect("error opening rar archive")
            .process()
            .expect("error extracting archive");
    }
}

fn main() {
    let current_directory = get_current_directory();
    let archive_list = scan_for_rar(&current_directory);
    println!("Easy RAR to CBZ");

    if archive_list.is_empty() {
    } else {
        println!("extracting");
        process_archives(archive_list);
    }
}
