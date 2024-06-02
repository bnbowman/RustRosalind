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

fn get_suffix_prefix_match(lhs: &fasta::Record, rhs: &fasta::Record, k: usize) -> i32 {
    if lhs.seq().len() < k || rhs.seq().len() < k {
        return 0;
    }
    let l_seq = str::from_utf8(lhs.seq()).unwrap();
    let r_seq = str::from_utf8(rhs.seq()).unwrap();

    let mut suffix = &l_seq[l_seq.len() - k..];
    if let Some(p) = r_seq.find(suffix) {
        // If we find the suffix, check whether it forms a prefix
        //  and return the full prefix-length if so
        suffix = &l_seq[l_seq.len() - k - p..];
        if r_seq.starts_with(suffix) {
            return (k + p) as i32;
        }
    }

    // If we either found no match, or couldn't extend
    //  the match to the beginning of r_seq, return -1
    return -1;
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let fasta_filepath = &args[1];

    let records = read_fasta(fasta_filepath.as_str());
    let min_len = records.iter().map(|r| r.seq().len()).min().unwrap();
    let min_ovl = (min_len as f32 / 2.0) as usize;
    println!("{:?}", records);
    println!("{:?}", min_len);
    println!("{:?}", min_ovl);

    for i in 1..records.len() {
        let ovl = get_suffix_prefix_match(&records[0], &records[i], min_ovl);
        println!("{} {} {}", records[0].id(), records[i].id(), ovl);
    }

    Ok(())
}
