use std::env;
use std::fs;
use std::io::BufReader;
use std::str;

use std::collections::HashMap;

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

fn get_largest_overlaps(
    records: &Vec<fasta::Record>,
    min_overlap: usize,
) -> (usize, Vec<(usize, usize)>) {
    let mut map: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();

    for i in 0..records.len() {
        for j in 0..records.len() {
            if i == j {
                continue;
            }
            let ovl = get_suffix_prefix_match(&records[i], &records[j], min_overlap);
            if ovl > 0 {
                if !map.contains_key(&ovl) {
                    map.insert(ovl, vec![(i, j)]);
                } else {
                    map.get_mut(&ovl).unwrap().push((i, j))
                }
            }
        }
    }

    let max_overlap = map.keys().map(|k| k.to_owned()).max().unwrap();
    let pairs = map.get(&max_overlap).unwrap().clone();
    let mut selected_indices: Vec<usize> = vec![];
    let mut selected_pairs: Vec<(usize, usize)> = vec![];
    for pair in pairs {
        if !selected_indices.contains(&pair.0) && !selected_indices.contains(&pair.1) {
            selected_pairs.push(pair);
            selected_indices.push(pair.0);
            selected_indices.push(pair.1);
        }
    }
    return (max_overlap as usize, selected_pairs);
}

fn merge_records(lhs: &fasta::Record, rhs: &fasta::Record, k: usize) -> fasta::Record {
    let l_seq = str::from_utf8(lhs.seq()).unwrap().to_string();
    let r_seq = str::from_utf8(rhs.seq()).unwrap();
    let seq = l_seq + &r_seq[k..];
    return fasta::Record::with_attrs(lhs.id(), None, seq.as_bytes());
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let fasta_filepath = &args[1];

    let mut records = read_fasta(fasta_filepath.as_str());
    let min_len = records.iter().map(|r| r.seq().len()).min().unwrap();
    let min_ovl = (min_len as f32 / 2.0) as usize;

    while records.len() > 1 {
        // Identify the largest overlap size and all overlap pairs
        let ovls = get_largest_overlaps(&records, min_ovl);
        let ovl = ovls.0;
        let pairs = ovls.1;

        // First we merge our read-pairs at their overlaps, while
        //  tracking the indices we're consuming
        let mut new_records: Vec<fasta::Record> = vec![];
        let mut paired_indices: Vec<usize> = vec![];
        for pair in pairs {
            let m = merge_records(&records[pair.0], &records[pair.1], ovl);
            new_records.push(m);
            paired_indices.push(pair.0);
            paired_indices.push(pair.1);
        }

        // Next we append any remaining un-merged reads to to our vector
        //  of merged reads, based on the indices we tracked above
        for (i, r) in records.iter().enumerate() {
            if !paired_indices.contains(&i) {
                new_records.push(r.clone());
            }
        }

        // Final we replace our old records with the merged subset
        records = new_records;
    }

    println!("{}", str::from_utf8(records[0].seq()).unwrap());

    Ok(())
}
