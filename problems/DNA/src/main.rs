use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();
    println!("{:?}", filepath);

    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        let mut counts = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
        for c in line.chars() {
            if counts.contains_key(&c) {
                *counts.get_mut(&c).unwrap() += 1;
            }
        }
        println!("{} {} {} {}", counts[&'A'],  counts[&'C'],  counts[&'G'],  counts[&'T']);

        line.clear();
    }

    Ok(())
}
