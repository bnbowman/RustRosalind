use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn fibd(n: usize, m: usize) -> usize {
    let mut v: Vec<usize> = vec![0; n + 2];
    v[0] = 0;
    v[1] = 1;
    for i in 1..(n + 1) {
        if i < m {
            v[i + 1] = v[i] + v[i - 1];
        } else if i == m {
            v[i + 1] = v[i] + v[i - 1] - v[i - m + 1];
        } else {
            v[i + 1] = v[i] + v[i - 1] - v[i - m];
        }
    }
    //println!("{:?}", v);
    return v[n];
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

        let parts: Vec<&str> = line.split_whitespace().collect();
        let n: usize = parts[0].parse::<usize>().unwrap();
        let k: usize = parts[1].parse::<usize>().unwrap();
        println!("{}", fibd(n, k));

        line.clear();
    }

    Ok(())
}
