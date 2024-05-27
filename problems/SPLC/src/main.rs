use std::env;
use std::str;
use std::string::String;

use bio::io::fasta;

/// Search a protein sequence for N-glycosylation motifs and return their
///  locations as a vector of integers
///
/// Arguments:
/// * `fasta_file`: A fasta file containing the protein sequence to search
fn read_fasta(fasta_file: &str) -> Vec<bio::io::fasta::Record> {
    let reader = fasta::Reader::from_file(fasta_file).expect("Unable to open fasta file");
    return reader.records()
        .filter_map(|r| r.ok())
        .collect::<Vec<_>>();
}

fn find_introns(
    template: &bio::io::fasta::Record,
    introns: &[bio::io::fasta::Record],
) -> Vec<(usize, usize)> {
    let mut retval = Vec::<(usize, usize)>::new();

    let template_seq = str::from_utf8(template.seq()).unwrap();
    for i in introns {
        let intron = str::from_utf8(i.seq()).unwrap();
        let mut v: Vec<(usize, usize)> = template_seq
            .match_indices(intron)
            .map(|(i, _)| (i, i + intron.len()))
            .collect();
        retval.append(&mut v);
    }

    retval.sort();
    return retval;
}

fn splice_sequence(
    template: &bio::io::fasta::Record,
    introns: Vec<(usize, usize)>,
) -> String {
    let regions: Vec<_> = introns
        .into_iter()
        .rev()
        .collect();
    println!("{:?}", regions);
    
    let mut seq = String::from_utf8(template.seq().to_vec()).unwrap();
    let mut len = seq.len();
    for r in regions {
        let start = r.0;
        let end = r.1;
        let pre = seq[0..start].to_owned();
        let i = seq[start..end].to_owned(); 
        let post = seq[end..len].to_owned();
        seq = format!("{}{}", pre, post);
        len = seq.len();
        println!("{} {} {}", pre, i, post);
    }
    return seq;
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let records = read_fasta(filepath.as_str());
    let template = &records[0];
    let intron_seqs = &records[1..];
    let introns = find_introns(template, intron_seqs);
    println!("{} {} {}", template.id(), intron_seqs.len(), introns.len());
    let rna = splice_sequence(template, introns);
    println!("{}", rna);

    Ok(())
}
