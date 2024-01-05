use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;
use std::convert::TryInto;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut operators = HashMap::new();  
    let mut commands = Vec::new();
    let mut command_id = 0;
    let mut command_eof = 0;

    for line in reader.lines() {
        let input = line?;
        commands.push(input);
        command_eof += 1;
    }

    loop {
        let operation = commands[command_id].split(" -> ").collect::<Vec<&str>>();
        let operand = operation[1].to_owned();
        let instr = operation[0].split(" ").collect::<Vec<&str>>();
        if instr.len() == 1 {
            let mut is_int = 1;
            for c in operation[0].chars() {
                if !c.is_numeric() {
                    is_int = 0;
                }
            }
            if is_int == 1 {
                let mut val: usize = operation[0].parse().unwrap();
                if operand == "b" && part == 2 {
                    val = 46065;
                }
                operators.insert(operand, val);
            }
            else {
                if operators.get(operation[0]).is_some() {
                    let val = operators.get(operation[0]).unwrap();
                    operators.insert(operand, *val);
                }
            }
        }
        else if instr.len() == 2 {
            if instr[0] == "NOT" {
                let mut val: u16;
                if operators.get(instr[1]).is_some() {
                    match operators.get(instr[1]) {
                        Some(gotten) => val = *gotten as u16,
                        None => val = 0
                    }
                val = !val;
                operators.insert(operand, val.try_into().unwrap());
                }
            }
        }
        else if instr.len() == 3 {
            if instr[0] == "1" || operators.get(instr[0]).is_some() {
                let val_l: u16;
                match operators.get(instr[0]) {
                    Some(gotten) => val_l = *gotten as u16,
                    None => val_l = 1
                }
                if instr[1] == "AND" || instr[1] == "OR" {
                    if operators.get(instr[2]).is_some() {
                        let val_r: u16;
                        match operators.get(instr[2]) {
                            Some(gotten) => val_r = *gotten as u16,
                            None => val_r = 0
                        }
                        if instr[1] == "AND" {
                            let val = val_l & val_r;
                            operators.insert(operand, val.try_into().unwrap());
                        }
                        else {
                            let val = val_l | val_r;
                            operators.insert(operand, val.try_into().unwrap());
                        }
                    }
                }
                else {
                    let s_size: usize = instr[2].parse().unwrap();
                    if instr[1] == "LSHIFT" {
                        let val = val_l << s_size;
                        operators.insert(operand, val.try_into().unwrap());
                    }
                    else if instr[1] == "RSHIFT" {
                        let val = val_l >> s_size;
                        operators.insert(operand, val.try_into().unwrap());
                    }
                }
            }
        }
        command_id += 1;
        if command_id >= command_eof {
            command_id = 0;
            if operators.get("a").is_some() {
                break;
            }
        }
    }

    println!("{}", operators.get("a").unwrap());
    Ok(())
}
