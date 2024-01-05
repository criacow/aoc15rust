use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);

    let mut code_chars = 0;
    let mut value_chars = 0;
    let mut new_chars = 0;

    for line in reader.lines() {
        let input = line?;
        let cc = input.len();
        let mut vc = 0;
        let mut skip = 0;
        let mut new_string: String = "\"\\\"".to_owned();
        for c in 1..cc-1 {
            if skip > 0 {
                skip -= 1;
            }
            else {
                if &input[c..c+1] == "\\" {
                    if &input[c..c+2] == "\\x" {
                        skip += 3;
                        let escaped = "\\\\".to_owned() + &input[c+1..c+4];
                        new_string.push_str(&escaped);
                    }
                    else if &input[c..c+2] == "\\\\" || &input[c..c+2] == "\\\"" {
                        skip += 1;
                        let escaped = "\\\\".to_owned() + &input[c..c+2];
                        new_string.push_str(&escaped);
                    }
                }
                else {
                    new_string.push_str(&input[c..c+1]);
                }
                vc += 1;
            }
        }
        new_string.push_str("\\\"\"");
        code_chars += cc;
        value_chars += vc;
        new_chars += new_string.len();
    }

    if part == 1 {
        println!("{}", code_chars - value_chars);
    }
    else {
        println!("{}", new_chars - code_chars);
    }
    Ok(())
}
