use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut section = 1;
    let mut molecules = HashMap::new();
    let mut orig_medicine: String = "".to_string();
    let mut new_medicine: Vec<String> = Vec::new();
    let mut mol_count = 0;
    let mut rnar_count = 0;
    let mut y_count = 0;

    for line in reader.lines() {
        let l_str = line?;
        if section == 2 {
            orig_medicine = l_str;
        }
        else if l_str == "" {
            section = 2;
        }
        else {
            let input = l_str.split(" => ").collect::<Vec<&str>>();
            let m_name = input[0].to_owned();
            let m_replace = input[1].to_owned();

            molecules.entry(m_name).or_insert(Vec::new()).push(m_replace);
        }
    }  
    for i in 0..orig_medicine.len() {
        let mut mol_name: &str = "";
        if orig_medicine.chars().nth(i).unwrap().is_uppercase() {
            if i < orig_medicine.len() - 1 {
                if orig_medicine.chars().nth(i+1).unwrap().is_lowercase() {
                    mol_name = &orig_medicine[i..i+2];
                    mol_count += 1;
                }
                else {
                    mol_name = &orig_medicine[i..i+1];
                    mol_count += 1;
                }
            }
            else {
                mol_name = &orig_medicine[i..i+1];
                mol_count += 1;
            }
        }
        if mol_name == "Rn" || mol_name == "Ar" {
            rnar_count += 1;
        }
        if mol_name == "Y" {
            y_count += 1;
        }
        if molecules.contains_key(mol_name)
        {
            let m_replaces = molecules.get(mol_name).unwrap();
            for j in m_replaces {
                let mut new_med: Vec<&str> = orig_medicine.split("").collect::<Vec<&str>>();
                new_med[i+1] = j;
                if mol_name.len() == 2 {
                    new_med[i+2] = "";
                }
                let new_string = new_med.into_iter().collect::<String>();
                if !new_medicine.contains(&new_string) {
                    new_medicine.push(new_string);
                }
            }
        }
    }
    if part == 1 {
        println!("{}", new_medicine.len());
    }
    else {
        // https://www.reddit.com/r/adventofcode/comments/3xflz8/comment/cy4etju/
        println!("{}", mol_count - rnar_count - 2 * y_count - 1);
    }
    Ok(())
}
