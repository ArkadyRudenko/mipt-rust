#![forbid(unsafe_code)]

use std::{fs::File, io::BufRead, io::BufReader, collections::HashSet};
use std::io::{Write};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let file1 = File::open(args[1].clone()).unwrap();
    let file2 = File::open(args[2].clone()).unwrap();

    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);

    let mut lines1 = HashSet::new();
    let mut lines2 = HashSet::new();

    for line1 in reader1.lines() {
        match line1 {
            Ok(line) => lines1.insert(line),
            _ => false,
        };
    }
    for line2 in reader2.lines() {
        match line2 {
            Ok(line) => lines2.insert(line),
            _ => false,
        };
    }

    for line in lines1 {
        if lines2.contains(&*line) {
            println!("{}", line);
        }
    }
}
