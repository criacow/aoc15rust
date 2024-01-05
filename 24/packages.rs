use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut sum = 0;
    let mut packages: Vec<usize> = Vec::new();

    for line in reader.lines() {
        let package = line?.parse().unwrap();
        sum += package;
        packages.push(package);
    }

    for n in 1..packages.len() {
        let mut least_qe = 99999999999;
        for i in (0..packages.len()).combinations(n) {
            let mut total = 0;
            let mut qe = 1;
            for j in &i {
                total += packages[*j];
                qe *= packages[*j];
            }
            if total == sum / (part + 2) {
                if qe < least_qe {
                    least_qe = qe;
                }
            }
        }
        if least_qe < 99999999999 {
            println!("{}", least_qe);
            break;
        }
    }
    Ok(())
}
