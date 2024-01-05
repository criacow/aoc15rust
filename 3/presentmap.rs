use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut present_map: Vec<Vec<i8>> = vec![];
    for y in 0..159 {
      present_map.push(vec![]);
      for _ in 0..159 {
          present_map[y].push(0);
      }
    }
    present_map[80][80] = 1;
    let mut house_count = 1;
    let mut x = 80;
    let mut y = 80;
    let mut robo_x = 80;
    let mut robo_y = 80;
    let mut whose_turn = 0;

    for line in reader.lines() {
        for char in line?.chars() {
            if char == '^' {
                if whose_turn == 0 {
                    y -= 1;
                } else {
                    robo_y -= 1;
                }
            }
            else if char == 'v' {
                if whose_turn == 0 {
                    y += 1;
                } else {
                    robo_y += 1;
                }
            }
            else if char == '<' {
                if whose_turn == 0 {
                    x -= 1;
                } else {
                    robo_x -= 1;
                }
            }
            else if char == '>' {
                if whose_turn == 0 {
                    x += 1;
                } else {
                    robo_x += 1;
                }
            }
            if whose_turn == 0 {
                if present_map[y][x] == 0 {
                    house_count += 1;
                }
                present_map[y][x] += 1;
                if part == 2 {
                    whose_turn = 1;
                }
           } else {
                if present_map[robo_y][robo_x] == 0 {
                    house_count += 1;
                }
                present_map[robo_y][robo_x] += 1;
                whose_turn = 0;
           }
//            println!("moving to {},{} count {}", x, y, house_count);
        }
    }
    println!("{}", house_count);
    Ok(())
}
