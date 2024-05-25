use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str;

use regex::Regex;

use bio::io::fasta;


fn read_fasta(filepath: &str) -> usize {
    let nglyco: Regex = Regex::new("N[^P][ST][^P]").unwrap();
    let mut bases = 0;

    let reader = fasta::Reader::from_file(filepath).unwrap();
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");

        let seq = str::from_utf8(record.seq()).unwrap();
        println!("{}", seq);
        let results = nglyco.find_iter(seq).map(|m| m.start() + 1).collect::<Vec<usize>>();
        println!("{:?}", results);

        bases += record.seq().len();
    }

    return bases;
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let mut filepath_parts = filepath.split("/").collect::<Vec<&str>>();
    filepath_parts.truncate(filepath_parts.len() - 1);
    let root = filepath_parts.join("/") + "/";

    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let l = line.trim_end().len();
        line.truncate(l);

        if line.len() > 0 {
            let fastapath = root.clone() + line.as_str() + ".fasta";
            let bases = read_fasta(fastapath.as_str());
            println!(
                "{} {}", fastapath, bases
            );
        }

        line.clear();
    }

    Ok(())
}
