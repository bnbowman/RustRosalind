use std::env;
use std::str;
use std::string::String;

use bio::io::fasta;
use phf::phf_map;

static DNA_CODON_TABLE: phf::Map<&'static str, &'static str> = phf_map! {
    "TTT" => "F",
    "TTC" => "F",
    "TTA" => "L",
    "TTG" => "L",
    "CTT" => "L",
    "CTC" => "L",
    "CTA" => "L",
    "CTG" => "L",
    "ATT" => "I",
    "ATC" => "I",
    "ATA" => "I",
    "ATG" => "M",
    "GTT" => "V",
    "GTC" => "V",
    "GTA" => "V",
    "GTG" => "V",
    
    "TCT" => "S",
    "TCC" => "S",
    "TCA" => "S",
    "TCG" => "S",
    "CCT" => "P",
    "CCC" => "P",
    "CCA" => "P",
    "CCG" => "P",
    "ACT" => "T",
    "ACC" => "T",
    "ACA" => "T",
    "ACG" => "T",
    "GCT" => "A",
    "GCC" => "A",
    "GCA" => "A",
    "GCG" => "A",
    
    "TAT" => "Y",
    "TAC" => "Y",
    "TAA" => "",
    "TAG" => "",
    "CAT" => "H",
    "CAC" => "H",
    "CAA" => "Q",
    "CAG" => "Q",
    "AAT" => "N",
    "AAC" => "N",
    "AAA" => "K",
    "AAG" => "K",
    "GAT" => "D",
    "GAC" => "D",
    "GAA" => "E",
    "GAG" => "E",
    
    "TGT" => "C",
    "TGC" => "C",
    "TGA" => "",
    "TGG" => "W",
    "CGT" => "R",
    "CGC" => "R",
    "CGA" => "R",
    "CGG" => "R",
    "AGT" => "S",
    "AGC" => "S",
    "AGA" => "R",
    "AGG" => "R",
    "GGT" => "G",
    "GGC" => "G",
    "GGA" => "G",
    "GGG" => "G",
};

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
    //println!("{:?}", regions);
    
    let mut seq = String::from_utf8(template.seq().to_vec()).unwrap();
    let mut len = seq.len();
    for r in regions {
        let start = r.0;
        let end = r.1;
        let pre = seq[0..start].to_owned();
        //let i = seq[start..end].to_owned(); 
        let post = seq[end..len].to_owned();
        seq = format!("{}{}", pre, post);
        len = seq.len();
        //println!("{} {} {}", pre, i, post);
    }
    return seq;
}

fn translate_rna(rna: String) -> String {
    let mut prot = String::new();
    for i in (0..rna.len()).step_by(3) {
        let codon = &rna[i..i+3];
        let optional = DNA_CODON_TABLE.get(codon);
        match optional {
            Some(aa) => {
                prot.push_str(aa);
            },
            None => println!("'{}' is not a valid codon!", codon),
        }
        //println!("{} {} {}", i, codon, prot);
    }
    return prot;
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let records = read_fasta(filepath.as_str());
    let template = &records[0];
    let intron_seqs = &records[1..];
    let introns = find_introns(template, intron_seqs);
    let rna = splice_sequence(template, introns);
    let protein = translate_rna(rna);
    println!("{}", protein);

    Ok(())
}
