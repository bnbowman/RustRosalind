use std::env;
use std::fs;
use std::io::{BufReader, BufRead, Write};
use std::str;

//use regex::Regex;

use bio::io::fasta;

/// Query the UniProt database to retreive a protein sequence, and return it
///  as a FASTA file
///
/// Arguments:
/// * `uniprot_query`: The gene or sequence name to query
#[tokio::main]
async fn query_unitprot(uniprot_query: &str) -> Result<String, Box<dyn std::error::Error>> {
    let uniprot_id: &str;
    if uniprot_query.contains('_') {
        uniprot_id = uniprot_query.split('_').collect::<Vec<_>>().first().unwrap();
    } else {
        uniprot_id = uniprot_query;
    }
    //println!("{}", uniprot_id);

    let rest_url = format!("{}{}{}", "https://rest.uniprot.org/uniprotkb/", uniprot_id, ".fasta");
    let response = reqwest::get(rest_url).await?;
    let body = response.text().await?;
    //println!("{}", body);

    let fasta_file = String::from("/tmp/uniprot.fasta");
    let mut temp_file = fs::File::create(fasta_file.as_str()).expect("Unable to create file");
    temp_file.write_all(body.as_bytes()).expect("Unable to write data");

    Ok(fasta_file)
}

/// Search a protein sequence for N-glycosylation motifs and return their
///  locations as a vector of integers
///
/// Arguments:
/// * `fasta_file`: A fasta file containing the protein sequence to search
fn identify_nglyco_motifs(fasta_file: &str) -> Vec<usize> {
    // Base "N[^P][ST][^P]" doesn't allow for overlapping matches.
    // Regex's like the one below would, but Rust doesn't support
    // look-around functionality in Regexs yet
    //let nglyco: Regex = Regex::new("(?=(N[^P][ST][^P]))").unwrap();

    let f = fs::File::open(fasta_file).expect("Unable to open file");
    let buf = BufReader::new(f);
    let reader = fasta::Reader::new(buf);
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");

        let seq = str::from_utf8(record.seq()).unwrap();
        let seq_len = seq.len() as usize;
        let mut results: Vec<usize> = vec![];
        for i in 0..(seq_len-4) {
            if seq[i..i+1] == *"N" && seq[i+1..i+2] != *"P" && (seq[i+2..i+3] == *"S" || seq[i+2..i+3] == *"T") && seq[i+3..i+4] != *"P" {
                results.push(i+1);
            }
        }
        //let results = nglyco.find_iter(seq).map(|m| m.start() + 1).collect::<Vec<usize>>();
        return results;
    }

    return Vec::<usize>::new();
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let mut filepath_parts = filepath.split("/").collect::<Vec<&str>>();
    filepath_parts.truncate(filepath_parts.len() - 1);

    let file = fs::File::open(filepath)?;
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
            let uniprot_data = query_unitprot(line.as_str()).unwrap();
            let pos = identify_nglyco_motifs(uniprot_data.as_str());

            if pos.len() > 0 {
                let pos_str = pos.into_iter().map(|i| i.to_string() + " ").collect::<String>();
                println!(
                    "{}\n{}", line, pos_str
                );
            }
        }

        line.clear();
    }

    Ok(())
}
