static PART: usize = 2;

fn battle_time(mut state: Vec<usize>, spellnum: usize) -> usize {
    let spells: Vec<Vec<usize>> = vec!
        [vec![ 53, 4, 0, 0, 0, 0, 0],
         vec![ 73, 2, 2, 0, 0, 0, 0],
         vec![113, 0, 0, 7, 0, 0, 6],
         vec![173, 0, 0, 0, 3, 0, 6],
         vec![229, 0, 0, 0, 0,101,5]];

    // player turn

    // state: php, pmp, bhp, bdm, e1, e1l, e2, e2l, e3, e3l
    if spells[spellnum][1] > 0 {
       if state[2] <= spells[spellnum][1] {
           return 0;
       }
       state[2] -= spells[spellnum][1];
    }
    if spells[spellnum][2] > 0 {
       state[0] += spells[spellnum][2];
    }
    if spells[spellnum][6] > 0 && state[5] == 0 {
       state[4] = spellnum;
       state[5] = spells[spellnum][6];
    }
    else if spells[spellnum][6] > 0 && state[7] == 0 {
        state[6] = spellnum;
        state[7] = spells[spellnum][6];
    }
    else if spells[spellnum][6] > 0 && state[9] == 0 {
        state[8] = spellnum;
        state[9] = spells[spellnum][6];
    }

    // boss turn - effects phase
    if state[5] > 0 {
        let snum = state[4];
        if spells[snum][4] > 0 {
            if state[2] <= spells[snum][4] {
                return 0;
            }
            state[2] -= spells[snum][4];
        }
        else if spells[snum][5] > 0 {
            state[1] += spells[snum][5];
        }
        state[5] -= 1;
        if state[5] == 0 {
            state[4] = 99;
        }
    }
    if state[7] > 0 {
        let snum = state[6];
        if spells[snum][4] > 0 {
            if state[2] <= spells[snum][4] {
                return 0;
            }
            state[2] -= spells[snum][4];
        }
        else if spells[snum][5] > 0 {
            state[1] += spells[snum][5];
        }
        state[7] -= 1;
        if state[7] == 0 {
            state[6] = 99;
        }
    }
    if state[9] > 0 {
        let snum = state[8];
        if spells[snum][4] > 0 {
            if state[2] <= spells[snum][4] {
                return 0;
            }
            state[2] -= spells[snum][4];
        }
        else if spells[snum][5] > 0 {
            state[1] += spells[snum][5];
        }
        state[9] -= 1;
        if state[9] == 0 {
            state[8] = 99;
        }
    }

    // boss turn
    let mut boss_dmg = state[3];
    if state[4] == 2 || state[6] == 2 || state[8] == 2 {
        boss_dmg -= spells[2][3];
    }
    if state[0] <= boss_dmg {
        return 9999;
    }
    state[0] -= boss_dmg;

    if PART == 2 {
        if state[0] <= 1 {
            return 9999;
        }
        state[0] -= 1;
    }

    // player effects
    if state[5] > 0 {
        let snum = state[4];
        if spells[snum][4] > 0 {
            if state[2] <= spells[snum][4] {
                return 0;
            }
            state[2] -= spells[snum][4];
        }
        else if spells[snum][5] > 0 {
            state[1] += spells[snum][5];
        }
        state[5] -= 1;
        if state[5] == 0 {
            state[4] = 99;
        }
    }
    if state[7] > 0 {
        let snum = state[6];
        if spells[snum][4] > 0 {
            if state[2] <= spells[snum][4] {
                return 0;
            }
            state[2] -= spells[snum][4];
        }
        else if spells[snum][5] > 0 {
            state[1] += spells[snum][5];
        }
        state[7] -= 1;
        if state[7] == 0 {
            state[6] = 99;
        }
    }
    if state[9] > 0 {
        let snum = state[8];
        if spells[snum][4] > 0 {
            if state[2] <= spells[snum][4] {
                return 0;
            }
            state[2] -= spells[snum][4];
        }
        else if spells[snum][5] > 0 {
            state[1] += spells[snum][5];
        }
        state[9] -= 1;
        if state[9] == 0 {
            state[8] = 99;
        }
    }

    // recursion time
    let mut mana = 9999;
    for i in 0..spells.len() {
        if spells[i][0] <= state[1] {
            if state[4] != i && state[6] != i && state[8] != i {
                let mut state_cast = state.clone();
                let mut cur_mana = spells[i][0];
                state_cast[1] -= cur_mana;
                cur_mana += battle_time(state_cast.clone(), i);
                if cur_mana < mana {
                    mana = cur_mana;
                }
            }
        }
    }
    return mana;
}

fn main() {
    let mut player_init_hp = 50;
    if PART == 2 {
        player_init_hp -= 1;
    }
    let player_init_mp = 500;
    let boss_init_hp = 58;
    let boss_init_dmg = 9;

    let mut mana_used = 9999;

    let spells: Vec<Vec<usize>> = vec!
      [vec![ 53, 4, 0, 0, 0, 0, 0],
       vec![ 73, 2, 2, 0, 0, 0, 0],
       vec![113, 0, 0, 7, 0, 0, 6],
       vec![173, 0, 0, 0, 3, 0, 6],
       vec![229, 0, 0, 0, 0,101,5]];

  // state: php, pmp, bhp, bdm, e1, e1l, e2, e2l, e3, e3l
    let state: Vec<usize> = vec![player_init_hp, player_init_mp, boss_init_hp, boss_init_dmg, 99, 0, 99, 0, 99, 0];

    for i in 0..spells.len() {
        let mut cur_mana = spells[i][0];
        let mut state_cast = state.clone();
        state_cast[1] -= cur_mana;
        cur_mana += battle_time(state_cast.clone(), i);
        if cur_mana < mana_used {
            mana_used = cur_mana;
        }
    }

    println!("{}", mana_used);
}
