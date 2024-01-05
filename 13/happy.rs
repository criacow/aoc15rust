use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut people = vec!["Alice", "Bob", "Carol", "David", "Eric", "Frank", "George", "Mallory"];
    if part == 2 {
        people.push("me");
    }

    let mut happiness = HashMap::new();  

    for line in reader.lines() {
        let l_str = line?;
        let edges = l_str.split(" ").collect::<Vec<&str>>();
        let from_person = edges[0];
        let to_p = String::from(edges[10]);
        let to_person = &to_p[0..to_p.len() - 1];
        let f_c = people.iter().position(|n| n == &from_person).unwrap();
        let t_c = people.iter().position(|n| n == &to_person).unwrap();
        let mut happy: i32 = edges[3].parse().unwrap();
        if edges[2] == "lose" {
            happy *= -1;
        }
        let f_t = format!("{},{}", f_c, t_c);
        let t_f = format!("{},{}", t_c, f_c);
        if happiness.contains_key(&f_t) {
            happy += happiness.get(&f_t).unwrap();
        }
//        println!("{} {} {}", f_c, t_c, happy);
        happiness.insert(
            f_t,
            happy,
        );
        happiness.insert(
            t_f,
            happy,
        );
    }

    let mut combos = (1..9).permutations(8);
    if part == 2 {
        combos = (1..10).permutations(9);
    }
//    println!("{:?}", happiness);
//    let combos = (1..5).permutations(4);
    let mut tgt_happy = -99999;
    for x in combos {
        let mut tot_happy = 0;
        for y in 0..x.len() {
            if y < x.len()-1 {
                let coord = format!("{},{}", x[y]-1, x[y+1]-1);
                let happy = happiness.get(&coord).unwrap();
                tot_happy += happy;
//                println!("{} {}", coord, happy);
            }
        }
        let coord = format!("{},{}", x[0]-1, x[x.len()-1]-1);
        let happy = happiness.get(&coord).unwrap();
        tot_happy += happy;
//        println!("{} {}", coord, happy);
//        println!("{}", tot_happy);
        if tot_happy > tgt_happy {
            tgt_happy = tot_happy;
        }
    }    
    println!("{}", tgt_happy);
    Ok(())
}
