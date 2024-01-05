extern crate factor;
use factor::factor_include::factor_include;

fn main() {
    let i = 884520;
    let f = factor_include(i);
    println!("{:?}", f);
    let mut tot = 0;
    for x in f {
        if x * 50 >= i {
             println!("{}", x);
             tot += x;
        }
    }
    println!("{}", tot * 11);
}
