extern crate factor;
use factor::factor_include::factor_include;

fn main() {
    let part = 2;

    let mut i = 1;
    loop {
        let f = factor_include(i);
        let mut presents = 0;
        for x in f {
            if part == 1 {
                presents += 10 * x;
            }
            else {
                if x * 50 >= i {
                    presents += 11 * x;
                }
            }
        }
        if presents >= 36000000 {
            println!("house {} presents {}", i, presents);
            break;
        }
        i += 1;
        if i % 25000 == 0 {
            println!("house {} not yet", i);
        }
    }
}
