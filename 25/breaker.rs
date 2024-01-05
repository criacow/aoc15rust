fn algo(prvs: usize) -> usize {
    return prvs * 252533 % 33554393;
}

fn main() {
    let row = 2981;
    let col = 3075;
    let mut cur = 20151125;
    let mut iter = 0;
    loop {
       iter += 1;
       cur = algo(cur);
       if cur == 20151125 {
           break;
       }
    }
    let start_row_no = row + col - 1;
    let overall_val = (start_row_no - 1) * start_row_no / 2 + col;
    let val_in_seq = overall_val % iter;
    for _ in 1..val_in_seq {
       cur = algo(cur);
    }
    println!("{}", cur);
}
