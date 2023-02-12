mod hasher;
use std::time::Instant;
use std::{fs, io::Write};
use structopt::StructOpt;
use walkdir::WalkDir;
#[derive(Debug, StructOpt)]
#[structopt(name = "file hasher", about = "Calculate hash.")]
struct Opt {
    /// Set worker threads
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "t", long = "threads", default_value = "1")]
    threads: usize,
}

fn main() {
    let opt = Opt::from_args();
    let start_time = Instant::now();
    let map = hasher::hasher_multi(WalkDir::new("."), opt.threads);
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
    let mut can_decrease_size = 0_usize;
    for (k, v) in &map {
        out1.write(format!("{}\n{}\n", k.0, k.1).as_bytes())
            .unwrap();
        for s in v {
            out1.write(format!("{}\n", s).as_bytes()).unwrap();
        }
        out1.write("\n".as_bytes()).unwrap();

        if v.len() == 1 {
            continue;
        }
        can_decrease_size += (v.len() - 1) * k.1;
        out2.write(format!("{}\n{}\n", k.0, k.1).as_bytes())
            .unwrap();
        for s in v {
            out2.write(format!("{}\n", s).as_bytes()).unwrap();
        }
        out2.write("\n".as_bytes()).unwrap();
    }
    out2.write(format!("{}\n", can_decrease_size).as_bytes())
        .unwrap();
}
