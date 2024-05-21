use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


fn fib(n: usize, k: usize) -> usize {
    let mut v: Vec<usize> = vec![0; n];
    v[0] = 1;
    v[1] = 1;
    for i in 2..n {
        v[i] = k * v[i-2] + v[i-1];
    }
    //println!("{:?}", v);
    return v[n-1];
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
        println!("{}", fib(n, k));

        line.clear();
    }

    Ok(())
}
