use std::{env, fs, io};

use unrar::Archive;

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

fn extract_rar(archives: Vec<String>, current_directory: &String) {
    for rar in archives {
        Archive::new(rar)
            .extract_to(current_directory.clone())
            .unwrap()
            .process()
            .unwrap();
    }
}

fn main() {
    let current_directory = get_current_directory();
    let archive_list = scan_for_rar(&current_directory);
    println!("Easy RAR to CBZ");

    if archive_list.is_empty() {
    } else {
        println!("extracting");
        extract_rar(archive_list, &current_directory);
    }
}
