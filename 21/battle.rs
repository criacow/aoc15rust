use std::io;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let part = 2;

    let damage = [4, 5, 6, 7, 8, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 0, 0, 0];
    let armor = [0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 0, 0, 0, 0, 0, 1, 2, 3];
    let prices = [8, 10, 25, 40, 74, 0, 13, 31, 53, 75, 102, 0, 0, 25, 50, 100, 20, 40, 80];
    let my_hp = 100;
    let boss_hp = 104;
    let boss_dam = 8;
    let boss_arm = 1;

    let mut tgt_cost = 9999;
    if part == 2 {
        tgt_cost = 0;
    }
    let it = (0..5).cartesian_product(5..11);
    for i in it {
      let rings = (11..19).combinations(2);
      for r in rings {
        let dam = damage[i.0] + damage[i.1] + damage[r[0]] + damage[r[1]];
        let arm = armor[i.0] + armor[i.1] + armor[r[0]] + armor[r[1]];
        let cost = prices[i.0] + prices[i.1] + prices[r[0]] + prices[r[1]];

        let mut my_dps = dam - boss_arm;
        if my_dps < 1 {
            my_dps = 1;
        }
        let mut boss_dps = boss_dam - arm;
        if boss_dps < 1 {
            boss_dps = 1;
        }
        let mut i_win = 0;
        let mut my_cur_hp = my_hp;
        let mut boss_cur_hp = boss_hp;
        loop {
           boss_cur_hp -= my_dps;
           if boss_cur_hp <= 0 {
               i_win = 1;
               break;
           }
           my_cur_hp -= boss_dps;
           if my_cur_hp <= 0 {
               break;
           }
        }
        if part == 1 {
            if i_win == 1 && cost < tgt_cost {
                tgt_cost = cost;
            } 
        }
        else {
            if i_win == 0 && cost > tgt_cost {
                tgt_cost = cost;
            } 
        }
//        println!("{} {} {} {}", dam, arm, cost, i_win);
      }
    }
    println!("{}", tgt_cost);
    Ok(())
}
