use crypto::digest::Digest;
use crypto::md5::Md5;
use crypto::sha2::Sha256;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

mod hasher;
mod oneOrMany;
fn main() {
    for entry in WalkDir::new("./") {
        break;
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            continue;
        }
        println!(
            "{}",
            match entry.file_name().to_str() {
                Some(s) => s,
                None => "",
            }
        );
        println!("{}", entry.path().display());
        println!("{:?}", entry.file_type().is_file())
    }

    println!("Hello, world!");

    let file = fs::read("./1.txt").unwrap();
    let fileu8: &[u8] = &file;
    let mut hasher = Md5::new();
    hasher.input(&file);
    let result = hasher.result_str();
    println!("{}", get_hash("./1.txt"));

    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("./2.txt")
        .unwrap();
    let f = file.write("aaa".as_bytes());

    use std::collections::HashMap;
    let mut h = HashMap::<u8, String>::new();
    h.insert(1, String::from("1"));
    h.insert(2, String::from("2"));
    for a in &mut h {
        a.1.insert(1, '2');
    }
    match h.get_mut(&1) {
        Some(s) => s.push('/'),
        None => {}
    };
    println!("{:?}", h);
}

fn get_hash<P: AsRef<Path>>(path: P) -> String {
    let file = fs::read(path).unwrap();
    let mut hasher = Md5::new();
    hasher.input(&file);
    hasher.result_str()
}
