use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut floor = 0;
    let mut position = 0;

    for line in reader.lines() {
        for (index, char) in line?.chars().enumerate() {
            if char == '(' {
               floor += 1;
            }
            else if char == ')' {
               floor -= 1;
               if part == 2 && position == 0 && floor == -1 {
                   position = index;
               }          
            }
        }
    }
    if part == 1 {
      println!("{}", floor);
    } else if part == 2 {
      println!("{}", position + 1);
    }
    Ok(())
}
