use std::io::prelude::*;
use std::io;
use std::env;
use std::path::PathBuf;
use std::fs::File;

fn main() {

    // Read the first command line argument
    let mut argv = env::args();
    let needle = match argv.nth(1) {
        None => {
            println!("No search argument given\nUsage: `iow wtf`");
            return;
        }
        Some(arg) => arg,
    };

    // Setting the search path for the dictionaries
    let iow_dir = {
        match env::var("IOW_DICT") {
            Ok(val) => PathBuf::from(val),
            Err(_)  => {
                let mut path = env::home_dir().unwrap();
                path.push(".iow");
                path
            }
        }
    };
    println!("Looking for dictionaries in: {:?}", iow_dir);

    let mut entry_found = false;

    let dir = iow_dir.read_dir().unwrap();
    for file in dir.filter_map(|f| f.ok()) {
        let file = File::open(file.path()).ok().unwrap();
        let reader = io::BufReader::new(file);
        for line in reader.lines().filter_map(|l| l.ok()) {
            let mut words: Vec<&str> = line.splitn(2, ' ').collect();
            let (a, d) = match (words.pop(), words.pop()) {
                (Some(d), Some(a)) => (a, d),
                (_, _) => ("", ""),
            };

            if a.to_uppercase() == needle.to_uppercase() {
                println!("{}: {}", a, d.trim());
                entry_found = true;
            }
        }
    }

    if !entry_found {
        println!("Definition for {} not found.", needle);
    }
}
