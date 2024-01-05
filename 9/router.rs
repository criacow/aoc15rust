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

    let places = vec!["Faerun", "Norrath", "Tristram", "AlphaCentauri", "Arbre", "Snowdin", "Tambi", "Straylight"];
    let mut distances = HashMap::new();  

    for line in reader.lines() {
        let l_str = line?;
        let edges = l_str.split(" ").collect::<Vec<&str>>();
        let from_city = edges[0];
        let to_city = edges[2];
        let f_c = places.iter().position(|n| n == &from_city).unwrap();
        let t_c = places.iter().position(|n| n == &to_city).unwrap();
        let distance: i32 = edges[4].parse().unwrap();
        let f_t = format!("{},{}", f_c, t_c);
        let t_f = format!("{},{}", t_c, f_c);
        distances.insert(
            f_t,
            distance,
        );
        distances.insert(
            t_f,
            distance,
        );
    }

    let combos = (1..9).permutations(8);
    let mut tgt_distance;
    if part == 1 {
        tgt_distance = 99999;
    }
    else {
        tgt_distance = 0;
    }
    for x in combos {
        let mut tot_distance = 0;
        for y in 0..x.len() {
            if y < x.len()-1 {
                let coord = format!("{},{}", x[y]-1, x[y+1]-1);
                let distance = distances.get(&coord).unwrap();
                tot_distance += distance;
            }
        }
        if part == 1 {
            if tot_distance < tgt_distance {
                tgt_distance = tot_distance;
            }
        }
        else {
            if tot_distance > tgt_distance {
                tgt_distance = tot_distance;
            }
        }
    }    
    println!("{}", tgt_distance);
    Ok(())
}
