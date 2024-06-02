use std::env;
use std::fs;
use std::io::BufReader;
use std::str;

use bio::io::fasta;

fn read_fasta(fasta_file: &str) -> Vec<fasta::Record> {
    let f = fs::File::open(fasta_file).expect("Unable to open file");
    let buf = BufReader::new(f);
    let reader = fasta::Reader::new(buf);
    return reader.records().map(|r| r.ok().unwrap()).collect();
}

pub fn factorial(num: u128) -> u128 {
    (1..=num).product()
}

static A_BYTE: u8 = 'A' as u8;
static G_BYTE: u8 = 'G' as u8;

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let fasta_filepath = &args[1];

    let records = read_fasta(fasta_filepath.as_str());

    for record in records {
        let a_count = record.seq().iter().filter(|c| **c == A_BYTE).count();
        let g_count = record.seq().iter().filter(|c| **c == G_BYTE).count();
        let a_factorial = factorial(a_count as u128);
        let g_factorial = factorial(g_count as u128);
        let perfect_matches = a_factorial * g_factorial;
        println!("{}", perfect_matches);
    }

    Ok(())
}
