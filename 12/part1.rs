use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use lazy_static::lazy_static;
use regex::Regex;

fn str_strip_numbers(s: &str) -> Vec<i64> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
    }
    // iterate over all matches
    RE.find_iter(s)
        // try to parse the string matches as i64 (inferred from fn type signature)
        // and filter out the matches that can't be parsed (e.g. if there are too many digits to store in an i64).
        .filter_map(|digits| digits.as_str().parse().ok())
        // collect the results in to a Vec<i64> (inferred from fn type signature)
        .collect()
}

fn main() -> io::Result<()> {
//    let part = 1;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);
    let mut sum = 0;
    for line in reader.lines() {
        let json = line?;
        let ints = str_strip_numbers(&json);
        for i in ints {
          sum += i;
        }
    }
    println!("{}", sum);
    Ok(())
}
