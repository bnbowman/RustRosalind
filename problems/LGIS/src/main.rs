use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let file = fs::File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut N: usize = 0;
    let mut X: Vec<i32> = Vec::<i32>::new();
    let mut P: Vec<i32> = Vec::<i32>::new();
    let mut M: Vec<i32> = Vec::<i32>::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let l = line.trim_end().len();
        line.truncate(l);

        if N == 0 {
            N = line.parse::<usize>().unwrap();
            M = vec![0; N+1];
            P = vec![-1; N];
        } else {
            X = line
                .split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
        }

        line.clear();
    }

    let mut L = 0usize;
    for i in (0..N) { //N-1 included
        // Binary search for the smallest positive l ≤ L
        // such that X[M[l]] > X[i]
        let mut lo = 1;
        let mut hi = L + 1;
        while lo < hi {
            let mid = (lo as f32 + ((hi-lo) as f32/2.0).floor()) as usize; // lo <= mid < hi
            if X[M[mid as usize] as usize] >= X[i] {
                hi = mid;
            } else { // if X[M[mid]] < X[i]
                lo = mid + 1;
            }
        }

        // After searching, lo == hi is 1 greater than the
        // length of the longest prefix of X[i]
        let newL = lo;

        // The predecessor of X[i] is the last index of 
        // the subsequence of length newL-1
        P[i] = M[newL-1];
        M[newL] = i as i32;
        
        if newL > L {
            // If we found a subsequence longer than any we've
            // found yet, update L
            L = newL;
        }
    }
    
    println!("{}", N);
    println!("{:?}", X);
    println!("{:?}", M);
    println!("{:?}", P);

    let mut S = vec![0i32; L];
    let mut k = M[L] as usize;
    for j in (0..L).rev() { //0 included
        S[j] = X[k];
        k = P[k] as usize;
    }

    println!("{:?}", S);
    
    Ok(())
}
