use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut reg_a: i64 = 0;
    let mut reg_b = 0;
    let mut commands = Vec::new();
    let mut command_id = 0;
    let mut command_eof = 0;

    if part == 2 {
        reg_a = 1;
    }

    for line in reader.lines() {
        let input = line?;
        commands.push(input);
        command_eof += 1;
    }

    loop {
        let c_split = commands[command_id].split(" ").collect::<Vec<&str>>();

        if c_split[0] == "hlf" {
            if c_split[1] == "a" {
                reg_a /= 2;
            }
            else if c_split[1] == "b" {
                reg_b /= 2;
            }
            command_id += 1;
        }
        else if c_split[0] == "tpl" {
            if c_split[1] == "a" {
                reg_a *= 3;
            }
            else if c_split[1] == "b" {
                reg_b *= 3;
            }
            command_id += 1;
        }
        else if c_split[0] == "inc" {
            if c_split[1] == "a" {
                reg_a += 1;
            }
            else if c_split[1] == "b" {
                reg_b += 1;
            }
            command_id += 1;
        }
        else if c_split[0] == "jmp" {
            let offset = c_split[1];
            let o_rows = &offset[1..].parse().unwrap();
            if &c_split[1][0..1] == "+" {
                command_id += o_rows;
            }
            else {
                command_id -= o_rows;
            }
        }
        else if c_split[0] == "jie" {
            if c_split[1] == "a," {
                if reg_a % 2 == 0 {
                    let offset = c_split[2];
                    let o_rows = &offset[1..].parse().unwrap();
                    if &c_split[2][0..1] == "+" {
                        command_id += o_rows;
                    }
                    else {
                        command_id -= o_rows;
                    }
                }
                else {
                    command_id += 1;
                }
            }
            else if c_split[1] == "b," {
                if reg_b % 2 == 0 {
                    let offset = c_split[2];
                    let o_rows = &offset[1..].parse().unwrap();
                    if &c_split[2][0..1] == "+" {
                        command_id += o_rows;
                    }
                    else {
                        command_id -= o_rows;
                    }
                }
            }
        }
        else if c_split[0] == "jio" {
            if c_split[1] == "a," {
                if reg_a == 1 {
                    let offset = c_split[2];
                    let o_rows = &offset[1..].parse().unwrap();
                    if &c_split[2][0..1] == "+" {
                        command_id += o_rows;
                    }
                    else {
                        command_id -= o_rows;
                    }
                }
                else {
                    command_id += 1;
                }
            }
            else if c_split[1] == "b," {
                if reg_b == 1 {
                    let offset = c_split[2];
                    let o_rows = &offset[1..].parse().unwrap();
                    if &c_split[2][0..1] == "+" {
                        command_id += o_rows;
                    }
                    else {
                        command_id -= o_rows;
                    }
                }
            }
        }

        if command_id >= command_eof {
            break;
        }
    }

    println!("a: {}, b: {}", reg_a, reg_b);
    Ok(())
}
