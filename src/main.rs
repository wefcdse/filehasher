mod hasher;
use std::time::Instant;
use std::{fs, io::Write};
use walkdir::WalkDir;
fn main() {
    let start_time = Instant::now();
    let map = hasher::hasher_multi(WalkDir::new("."), 8);
    println!("Finished in {}ms", start_time.elapsed().as_millis());
    let mut out1 = fs::OpenOptions::new()
        .create(true)
        .append(false)
        .write(true)
        .open("./1.txt")
        .unwrap();
    let mut out2 = fs::OpenOptions::new()
        .create(true)
        .append(false)
        .write(true)
        .open("./2.txt")
        .unwrap();
    for (k, v) in &map {
        out1.write(format!("{}\n", k).as_bytes()).unwrap();
        for s in v {
            out1.write(format!("{}\n", s).as_bytes()).unwrap();
        }
        out1.write("\n".as_bytes()).unwrap();

        if v.len() == 1 {
            continue;
        }
        out2.write(format!("{}\n", k).as_bytes()).unwrap();
        for s in v {
            out2.write(format!("{}\n", s).as_bytes()).unwrap();
        }
        out2.write("\n".as_bytes()).unwrap();
    }
}
