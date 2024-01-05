use md5;
//use hex_literal::hex;

fn main() {
  let part = 2;
  let mystr = "yzbqklnj";

  let mut i = 0;
  loop {
    let checkstr = mystr.to_owned() + &i.to_string();
    let digest = md5::compute(checkstr);
    let hexstr = format!("{:x}", digest);
    if part == 1 {
      if &hexstr[..5] == "00000" {
        break;
      }
    } else {
      if &hexstr[..6] == "000000" {
        break;
      }
    }
//    if i % 10000 == 0 {
//      println!("{}", i);
//    }
    i += 1;
  }
  println!("{}", i);
}
