use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut nice_strings = 0;

    for line in reader.lines() {
        let input = line?;
        let mut num_vowels = 0;
        let mut has_double = 0;
        let mut has_bad = 0;
        let mut has_pair = 0;
        let mut has_aba = 0;
        for c in 0..input.len() {
            if &input[c..c+1] == "a" || &input[c..c+1] == "e" || &input[c..c+1] == "i" || &input[c..c+1] == "o" || &input[c..c+1] == "u" {
                num_vowels += 1;
            }
            if c != input.len() - 1 {
                if &input[c..c+1] == &input[c+1..c+2] {
                    has_double = 1;
                }
            }
            if c < input.len() - 2 {
                if &input[c..c+1] == &input[c+2..c+3] {
                   has_aba = 1;
                }
            }
            if c < input.len() - 3 {
                for d in c+2..input.len()-1 {
                    if &input[c..c+2] == &input[d..d+2] {
                        has_pair = 1;
                    }
                }
            }
        }
        if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy") {
            has_bad = 1;
        }
        if part == 1 {
            if num_vowels >= 3 && has_double == 1 && has_bad == 0 {
                nice_strings += 1;
            }
        }
        else {
            if has_pair == 1 && has_aba == 1 {
                nice_strings += 1;
            }
        }
    }
    println!("{}", nice_strings);
    Ok(())
}
