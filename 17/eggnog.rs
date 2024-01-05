use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let part = 2;
    let target = 150;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut containers: Vec<usize> = Vec::new();
    let mut good_nog = 0;
    if part == 2 {
//        people.push("me");
    }

    for line in reader.lines() {
        let container = line?.parse().unwrap();
        containers.push(container);
    }

    let mut combo_containers: Vec<usize> = Vec::new();
    for _ in 0..containers.len()+1 {
        combo_containers.push(0);
    }
    for i in 0..containers.len()+1 {
        let combos = (1..containers.len()+1).combinations(i+1);
        for x in combos {
            let mut nog = 0;
            for y in x {
                nog += containers[y-1];
            }
            if nog == target {
                if part == 1 {
                    good_nog += 1;
                }
                else {
                    combo_containers[i] += 1;
                }
            }
        }
        if combo_containers[i] > 0 {
            good_nog = combo_containers[i];
            break;
        }
    }
    println!("{}", good_nog);
    Ok(())
}
