use std::env;
use std::fs;
use std::io::{BufRead, BufReader};

struct Lexicon {
    curr: String,
    next: String,
    last: String,
    alphabet: Vec<char>,
}

// Implement `Iterator` for `Lexicon`.
impl Iterator for Lexicon {
    // We can refer to this type using Self::Item
    type Item = String;

    // The return type is `Option<Item>`:
    //     * When the `Iterator` is finished, `None` is returned.
    //     * Otherwise, the next value is wrapped in `Some` and returned.
    fn next(&mut self) -> Option<Self::Item> {
        if self.next == "" {
            None // Handle last case
        } else if self.curr == self.last {
            // Set-up the end of the Iterator
            self.next = String::from("");
            Some(self.last.clone())
        } else {
            // Main iteration case
            let current = self.curr.clone();
            self.curr = self.next.clone();

            let mut new_next: Vec<char> = self.next.chars().collect();
            for i in (0..self.next.len()).rev() {
                let ith_char = new_next[i];
                let char_rank = self.alphabet.iter().position(|&c| c == ith_char).unwrap();
                if char_rank < self.alphabet.len() - 1 {
                    new_next[i] = self.alphabet[char_rank + 1];
                    break;
                } else {
                    new_next[i] = self.alphabet[0];
                }
            }
            self.next = new_next.iter().collect();

            Some(current)
        }
    }
}

// Returns a Lexicographical generator
fn lexicon(alphabet: String, len: usize) -> Lexicon {
    let alpha: Vec<char> = alphabet.chars().collect();
    let first_c = alpha[0];
    let second_c = alpha[1];
    let last_c = alpha[alpha.len() - 1];

    let first: String = vec![first_c; len].iter().collect();
    let mut next: String = vec![first_c; len - 1].iter().collect();
    next.push(second_c);
    let last: String = vec![last_c; len].iter().collect();

    return Lexicon {
        curr: first,
        next: next,
        last: last,
        alphabet: alpha,
    };
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().collect();
    let filepath: String = args[1].clone();

    let file = fs::File::open(filepath)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut alpha: String = String::from("");
    let mut len: usize = 0;
    loop {
        let bytes_read = reader.read_line(&mut line)?;
        if bytes_read == 0 {
            break;
        }
        let l = line.trim_end().len();
        line.truncate(l);

        if alpha.len() == 0 {
            alpha = line
                .split_whitespace()
                .map(String::from)
                .collect::<Vec<String>>()
                .join("");
        } else if len == 0 {
            len = line.parse::<usize>().unwrap();
        }

        line.clear();
    }

    let mut lexicon: Lexicon = lexicon(alpha, len);

    while let Some(word) = lexicon.next() {
        println!("{}", word);
    }

    Ok(())
}
