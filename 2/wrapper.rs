use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut sqft_paper = 0;
    let mut ribbon_feet = 0;

    for line in reader.lines() {
        let l_str = line?;
        let dimensions = l_str.split("x").collect::<Vec<&str>>();
        let length: i32 = dimensions[0].parse().unwrap();
        let width: i32 = dimensions[1].parse().unwrap();
        let height: i32 = dimensions[2].parse().unwrap();
        let area = 2 * length * width + 2 * width * height + 2 * height * length;

        let mut slack = length * width;
        if (length * height) < slack {
            slack = length * height;
        }
        if (width * height) < slack {
            slack = width * height;
        }

        let mut ribbon = length + width;
        if (length + height) < ribbon {
            ribbon = length + height;
        }
        if (width + height) < ribbon {
            ribbon = width + height;
        }
        ribbon *= 2;
        ribbon += width * length * height;
        
//        println!("{}  area: {}, slack: {}, ribbon: {}", l_str, area, slack, ribbon);
        sqft_paper += area + slack;
        ribbon_feet += ribbon;
    }
    if part == 1 {
      println!("{}", sqft_paper);
    } else if part == 2 {
      println!("{}", ribbon_feet);
    }
    Ok(())
}
