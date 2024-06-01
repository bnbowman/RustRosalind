use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn iev(counts: Vec<u32>) -> f32 {
    let mut total: f32 = 0.0;
    let mut fraction: f32;
    for (index, count) in counts.iter().enumerate() {
        if index < 3 {
            fraction = 1.0;
        } else if index == 3 {
            fraction = 0.75;
        } else if index == 4 {
            fraction = 0.50;
        } else {
            fraction = 0.0;
        }
        total += *count as f32 * fraction * 2.0;
    }
    return total;
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let file = File::open(filepath)?;
    let mut reader = BufReader::new(file);

    let mut line = String::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }

        let counts: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
        println!("{}", iev(counts));

        line.clear();
    }

    Ok(())
}
