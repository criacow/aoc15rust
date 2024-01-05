fn main() {
    let part = 2;

    let mut max_score = 0;

    for i in 1..100 {
        for j in 1..100 {
            for k in 1..100 {
                for l in 1..100 {
                    if i + j + k + l != 100 {
                        continue;
                    }
                    if part == 2 {
                        let calories = 3 * i + 3 * j + 8 * k + 8 * l;
                        if calories != 500 {
                            continue;
                        }
                    }
                    let mut capacity = 2 * i;
                    let mut durability = 5 * j - l;
                    let mut flavour = -2 * i + 5 * k - 3 * j;
                    let mut texture = 5 * l - k;
                    if capacity < 0 {
                        capacity = 0;
                    }
                    if durability < 0 {
                        durability = 0;
                    }
                    if flavour < 0 {
                        flavour = 0;
                    }
                    if texture < 0 {
                        texture = 0;
                    }
                    let score = capacity * durability * flavour * texture;

                    if score > max_score {
                        max_score = score;
                    }
                }
            }
        }
    }

    println!("{}", max_score);
}
