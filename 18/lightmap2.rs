use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;
    let num_steps = 100;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut light_map: Vec<Vec<usize>> = vec![];

    for line in reader.lines() {
        let input = line?;
        let mut light_row: Vec<usize> = vec![];
        for i in input.chars() {
            if i == '#' {
                light_row.push(1);
            }
            else if i == '.' {
                light_row.push(0);
            }
        }
        light_map.push(light_row);
    }

    if part == 2 {
        let y = light_map.len() - 1;
        let x = light_map[y].len() - 1;
        light_map[0][0] = 1;
        light_map[0][x] = 1;
        light_map[y][x] = 1;
        light_map[y][0] = 1;
    }

    for _ in 0..num_steps {
        let mut new_map = light_map.clone();
        for y in 0..light_map.len() {
            for x in 0..light_map[y].len() {
                let mut neighbours = vec![];
                if x == 0 && y == 0 {
                    neighbours.push([1,0]);
                    neighbours.push([1,1]);
                    neighbours.push([0,1]);
                }
                else if x == 0 && y == light_map.len() - 1 {
                    neighbours.push([x,y-1]);
                    neighbours.push([x+1,y-1]);
                    neighbours.push([x+1,y]);
                }
                else if x == light_map[y].len() - 1 && y == 0 {
                    neighbours.push([x,y+1]);
                    neighbours.push([x-1,y+1]);
                    neighbours.push([x-1,y]);
                }
                else if x == light_map[y].len() - 1 && y == light_map.len() - 1 {
                    neighbours.push([x,y-1]);
                    neighbours.push([x-1,y-1]);
                    neighbours.push([x-1,y]);
                }
                else if x == 0 {
                    neighbours.push([x,y+1]);
                    neighbours.push([x+1,y+1]);
                    neighbours.push([x+1,y]);
                    neighbours.push([x+1,y-1]);
                    neighbours.push([x,y-1]);
                }
                else if x == light_map[y].len() - 1 {
                    neighbours.push([x,y+1]);
                    neighbours.push([x-1,y+1]);
                    neighbours.push([x-1,y]);
                    neighbours.push([x-1,y-1]);
                    neighbours.push([x,y-1]);
                }
                else if y == 0 {
                    neighbours.push([x+1,y]);
                    neighbours.push([x+1,y+1]);
                    neighbours.push([x,y+1]);
                    neighbours.push([x-1,y+1]);
                    neighbours.push([x-1,y]);
                }
                else if y == light_map.len() - 1 {
                    neighbours.push([x+1,y]);
                    neighbours.push([x+1,y-1]);
                    neighbours.push([x,y-1]);
                    neighbours.push([x-1,y-1]);
                    neighbours.push([x-1,y]);
                }
                else {
                    neighbours.push([x+1,y]);
                    neighbours.push([x+1,y+1]);
                    neighbours.push([x,y+1]);
                    neighbours.push([x-1,y+1]);
                    neighbours.push([x-1,y]);
                    neighbours.push([x-1,y-1]);
                    neighbours.push([x,y-1]);
                    neighbours.push([x+1,y-1]);
                }
                let mut count_on = 0;
                for n in neighbours {
                    if light_map[n[1]][n[0]] == 1 {
                        count_on += 1;
                    }
                }
                if light_map[y][x] == 1 && count_on != 2 && count_on != 3 {
                    new_map[y][x] = 0;
                }
                if light_map[y][x] == 0 && count_on == 3 {
                    new_map[y][x] = 1;
                }
                if part == 2 {
                    new_map[0][0] = 1;
                    new_map[0][light_map[y].len()-1] = 1;
                    new_map[light_map.len()-1][light_map[y].len()-1] = 1;
                    new_map[light_map.len()-1][0] = 1;
                }
            }
        }
        light_map = new_map.clone();
    }

    let mut count = 0;
    for y in 0..light_map.len() {
        for x in 0..light_map[y].len() {
            if light_map[y][x] == 1 {
                count += 1;
            }
        }
    }
    println!("{}", count);
    Ok(())
}
