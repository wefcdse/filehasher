#![allow(dead_code)]

use crypto::digest::Digest;
use crypto::md5::Md5;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use walkdir;
use walkdir::WalkDir;
pub fn hasher(dir: WalkDir) -> HashMap<(String, usize), Vec<String>> {
    let mut map = HashMap::<(String, usize), Vec<String>>::new();
    for entry in dir {
        let entry = match entry {
            Ok(f) => {
                if f.file_type().is_file() {
                    f
                } else {
                    continue;
                }
            }
            Err(e) => {
                println!("[ERROR]{}", e);
                continue;
            }
        };

        let hash = match get_hash(entry.path()) {
            Ok(h) => h,
            Err(e) => {
                println!("[ERROR]{}", e);
                continue;
            }
        };

        match map.get_mut(&hash) {
            Some(v) => v.push((&entry.path().display()).to_string()),
            None => {
                let v = vec![(&entry.path().display()).to_string()];
                map.insert(hash, v);
            }
        };
    }
    map
}

fn get_hash<P: AsRef<Path>>(path: P) -> Result<(String, usize), std::io::Error> {
    let file = fs::read(path)?;
    let size = file.len();
    let mut hasher = Md5::new();
    hasher.input(&file);
    Ok((hasher.result_str(), size))
}

#[test]
fn t1() -> () {
    let map = hasher(WalkDir::new("./t"));
    println!("{:?}", map);
}

use crossbeam_channel;
use crossbeam_channel::unbounded;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
#[derive(Debug, Clone)]
struct WorkResult {
    hash: (String, usize),

    path: String,
    id: usize,
}
#[derive(Debug, Clone)]
enum Workload {
    New(String),
    End,
}
pub fn hasher_multi(dir: WalkDir, threads: usize) -> HashMap<(String, usize), Vec<String>> {
    let mut map = HashMap::<(String, usize), Vec<String>>::new();

    let (tx_result, rx_result) = mpsc::channel::<WorkResult>();
    let mut thread_pool: Vec<JoinHandle<()>> = Vec::new();
    let (tx_work, rx_work) = unbounded::<Workload>();
    for id in 0..threads {
        let tx_result = tx_result.clone();
        let rx_work = rx_work.clone();
        let handle = thread::spawn(move || {
            let id = id;
            let tx_result_thread = tx_result.clone();
            let rx_work_thread = rx_work.clone();
            for work in rx_work_thread {
                let path = match work {
                    Workload::New(s) => s,
                    Workload::End => {
                        return;
                    }
                };
                let hash = match get_hash(&path) {
                    Ok(s) => s,
                    Err(e) => {
                        println!("[ERROR]{}", e);
                        continue;
                    }
                };
                let result = WorkResult { hash, path, id };
                match tx_result_thread.send(result) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("[ERROR]{}", e);
                    }
                };
            }
        });

        thread_pool.push(handle);
    }
    drop(tx_result);
    drop(rx_work);

    //send works
    for entry in dir {
        let entry = match entry {
            Ok(f) => {
                if f.file_type().is_file() {
                    f
                } else {
                    continue;
                }
            }
            Err(e) => {
                println!("[ERROR]{}", e);
                continue;
            }
        };
        let workload = Workload::New(entry.path().display().to_string());
        tx_work.send(workload).unwrap();
    }

    //stop all threads,and then the rx_result will close
    for _ in 0..threads * 2 {
        match tx_work.send(Workload::End) {
            Ok(_) => {}
            Err(_) => {
                //println!("{}", e);
            }
        };
    }

    //collect results
    for result in rx_result {
        let WorkResult {
            hash,
            id: _id,
            path,
        } = result;

        match map.get_mut(&hash) {
            Some(v) => v.push(path),
            None => {
                let v = vec![path];
                map.insert(hash, v);
            }
        };
    }

    //println!("OK");
    for h in thread_pool {
        h.join().unwrap();
    }

    map
}
#[test]
fn t2() -> () {
    let map = hasher_multi(WalkDir::new("./t"), 5);
    println!("{:?}", map);
}
