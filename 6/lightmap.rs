use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut light_map: Vec<Vec<i8>> = vec![];
    for y in 0..1000 {
      light_map.push(vec![]);
      for _ in 0..1000 {
          light_map[y].push(0);
      }
    }

    for line in reader.lines() {
        let input = line?;
        let i_split = input.split(" ").collect::<Vec<&str>>();

        if &input[..7] == "turn of" {
            let startcoord = i_split[2].split(",").collect::<Vec<&str>>();
            let start_x: usize = startcoord[0].parse().unwrap();
            let start_y: usize = startcoord[1].parse().unwrap();
            let endcoord = i_split[4].split(",").collect::<Vec<&str>>();
            let end_x: usize = endcoord[0].parse().unwrap();
            let end_y: usize = endcoord[1].parse().unwrap();          

            for y in start_y..end_y+1 {
                for x in start_x..end_x+1 {
                    if part == 1 {
                        light_map[y][x] = 0;
                    }
                    else {
                        if light_map[y][x] > 0 {
                            light_map[y][x] -= 1;
                        }
                    }
                }
            }
        } else if &input[..7] == "turn on" {
            let startcoord = i_split[2].split(",").collect::<Vec<&str>>();
            let start_x: usize = startcoord[0].parse().unwrap();
            let start_y: usize = startcoord[1].parse().unwrap();
            let endcoord = i_split[4].split(",").collect::<Vec<&str>>();
            let end_x: usize = endcoord[0].parse().unwrap();
            let end_y: usize = endcoord[1].parse().unwrap();          

            for y in start_y..end_y+1 {
                for x in start_x..end_x+1 {
                    if part == 1 {
                        light_map[y][x] = 1;
                    }
                    else {
                        light_map[y][x] += 1;
                    }
                }
            }
        } else if &input[..7] == "toggle " {
            let startcoord = i_split[1].split(",").collect::<Vec<&str>>();
            let start_x: usize = startcoord[0].parse().unwrap();
            let start_y: usize = startcoord[1].parse().unwrap();
            let endcoord = i_split[3].split(",").collect::<Vec<&str>>();
            let end_x: usize = endcoord[0].parse().unwrap();
            let end_y: usize = endcoord[1].parse().unwrap();          

            for y in start_y..end_y+1 {
                for x in start_x..end_x+1 {
                    if part == 1 {
                        if light_map[y][x] == 0 {
                            light_map[y][x] = 1;
                        }
                        else {
                            light_map[y][x] = 0;
                        }
                    }
                    else {
                        light_map[y][x] += 2;
                    }
                }
            }
        }
    }

//            println!("moving to {},{} count {}", x, y, house_count);
    let mut count: i32 = 0;
    for y in 0..1000 {
        for x in 0..1000 {
//            println!("count: {}, light: {}", count, light_map[y][x]);
            count += i32::from(light_map[y][x]);
        }
    }
    println!("{}", count);
    Ok(())
}
