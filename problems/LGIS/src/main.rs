use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

fn get_subsequence(
    vec_x: &Vec<usize>,
    l: usize,
    first_index: usize,
    vec_p: Vec<usize>,
) -> Vec<usize> {
    let mut vec_s = vec![0usize; l];
    let mut k = first_index;
    for i in (0..l).rev() {
        vec_s[i] = vec_x[k] as usize;
        k = vec_p[k] as usize;
    }

    return vec_s;
}

fn get_midpoint(lo: usize, hi: usize) -> usize {
    return (lo as f32 + ((hi - lo) as f32 / 2.0).floor()) as usize;
}

fn longest_increasing_subsequence(vec_x: &Vec<usize>) -> Vec<usize> {
    let n = vec_x.len();
    let mut vec_m = vec![0usize; n + 1];
    let mut vec_p = vec![0usize; n];
    let mut l = 0usize;
    for i in 0..n {
        // Binary search for the smallest positive l ≤ L
        // such that X[M[l]] > X[i]
        let mut lo = 1;
        let mut hi = l + 1;
        while lo < hi {
            let mid = get_midpoint(lo, hi); // lo <= mid < hi
            if vec_x[vec_m[mid as usize] as usize] >= vec_x[i] {
                hi = mid;
            } else {
                // if X[M[mid]] < X[i]
                lo = mid + 1;
            }
        }

        // After searching, lo == hi is 1 greater than the
        // length of the longest prefix of X[i]
        let new_l = lo;

        // The predecessor of X[i] is the last index of
        // the subsequence of length newL-1
        vec_p[i] = vec_m[new_l - 1];
        vec_m[new_l] = i;

        if new_l > l {
            // If we found a subsequence longer than any we've
            // found yet, update L
            l = new_l;
        }
    }

    return get_subsequence(vec_x, l, vec_m[l], vec_p);
}

fn longest_decreasing_subsequence(vec_x: &Vec<usize>) -> Vec<usize> {
    let n = vec_x.len();
    let mut vec_m = vec![0usize; n + 1];
    let mut vec_p = vec![0usize; n];
    let mut l = 0usize;
    for i in 0..n {
        // Binary search for the smallest positive l ≤ L
        // such that X[M[l]] > X[i]
        let mut lo = 1;
        let mut hi = l + 1;
        while lo < hi {
            let mid = get_midpoint(lo, hi); // lo <= mid < hi
            if vec_x[vec_m[mid as usize] as usize] <= vec_x[i] {
                hi = mid;
            } else {
                // if X[M[mid]] < X[i]
                lo = mid + 1;
            }
        }

        // After searching, lo == hi is 1 greater than the
        // length of the longest prefix of X[i]
        let new_l = lo;

        // The predecessor of X[i] is the last index of
        // the subsequence of length newL-1
        vec_p[i] = vec_m[new_l - 1];
        vec_m[new_l] = i;

        if new_l > l {
            // If we found a subsequence longer than any we've
            // found yet, update L
            l = new_l;
        }
    }

    return get_subsequence(vec_x, l, vec_m[l], vec_p);
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let file = fs::File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut n: usize = 0;
    let mut vec_x: Vec<usize> = Vec::<usize>::new();
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let l = line.trim_end().len();
        line.truncate(l);

        if n == 0 {
            n = line.parse::<usize>().unwrap();
        } else {
            vec_x = line
                .split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
        }

        line.clear();
    }

    let vec_i = longest_increasing_subsequence(&vec_x);
    let vec_d = longest_decreasing_subsequence(&vec_x);

    let i = vec_i
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    let d = vec_d
        .iter()
        .map(|d| d.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("{}", i);
    println!("{}", d);

    Ok(())
}
