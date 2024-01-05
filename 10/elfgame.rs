fn main() {
  let part = 2;
  let mut mystr: String = "1113222113".to_string();
  let num_iterations = 30 + part * 10;

  for _ in 0..num_iterations {
    let mut new_string: String = "".to_string();
    let mut cur_count = 1;
    let mut cur_digit: char = 'x';
    for j in mystr.chars() {
      if j == cur_digit {
        cur_count += 1;
      }
      else {
        if cur_digit != 'x' {
            let concat = format!("{cur_count}{cur_digit}");
            new_string.push_str(&concat);
        }
        cur_count = 1;
        cur_digit = j;
      }
    }
    let concat = format!("{cur_count}{cur_digit}");
    new_string.push_str(&concat);
    mystr = new_string.clone();
  }
  println!("{}", mystr.len());
}
