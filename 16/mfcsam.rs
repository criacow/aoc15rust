use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let scan_results: HashMap<&str, usize> = HashMap::from([
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]);
    let mut real_sue = 0;

    for line in reader.lines() {
        let l_str = line?;
        let aunt_sue = l_str.split(" ").collect::<Vec<&str>>();
        let mut sue_i = aunt_sue[1];
        sue_i = &sue_i[..sue_i.len()-1];
        let sue_id = sue_i.parse().unwrap();
        let mut matcher = 1;
        for x in (2..aunt_sue.len()).step_by(2) {
            let mut thing_type = aunt_sue[x];
            thing_type = &thing_type[..thing_type.len()-1];
            let mut thing_a = aunt_sue[x+1];
            if x + 1 != aunt_sue.len() - 1 {
              thing_a = &thing_a[..thing_a.len()-1];
            }
            let thing_amt: usize = thing_a.parse().unwrap();

            if scan_results.contains_key(thing_type) {
                if part == 1 {
                    if thing_amt != *scan_results.get(thing_type).unwrap() {
                        matcher = 0;
                        break;
                    }
                }
                else {
                    if thing_type == "cats" || thing_type == "trees" {
                        if thing_amt <= *scan_results.get(thing_type).unwrap() {
                            matcher = 0;
                            break;
                        }
                    }
                    else if thing_type == "pomeranians" || thing_type == "goldfish" {
                        if thing_amt >= *scan_results.get(thing_type).unwrap() {
                            matcher = 0;
                            break;
                        }
                    }
                    else {
                        if thing_amt != *scan_results.get(thing_type).unwrap() {
                            matcher = 0;
                            break;
                        }
                    }
                }
            }
        }
        if matcher == 1 {
            real_sue = sue_id;        
        }
    }  
    println!("{}", real_sue);
    Ok(())
}
