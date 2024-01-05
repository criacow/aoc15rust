use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn main() -> io::Result<()> {
    let part = 2;
    let seconds = 2503;
    let mut max_dist = 0;

    let f = File::open("test.txt")?;
    let reader = BufReader::new(f);
    let mut data = Vec::new();

    for line in reader.lines() {
        let l_str = line?;
        data.push(l_str);
    }

    if part == 1 {
        for line in data {
            let input = line.split(" ").collect::<Vec<&str>>();
            let speed: usize = input[3].parse().unwrap();
            let go_time: usize = input[6].parse().unwrap();
            let rest_time: usize = input[13].parse().unwrap();
 
            let loop_time = go_time + rest_time;
            let full_loops = seconds / loop_time;
            let rem_loops = seconds % loop_time;
            let distance;
            if rem_loops >= go_time {
                distance = go_time * speed * (full_loops + 1);
            }
            else {
                distance = (go_time * full_loops + rem_loops) * speed;
            }
            if distance > max_dist {
                max_dist = distance;
            }
        }
    }
    else {
        let mut scores: Vec<usize> = Vec::new();
        for _ in 0..data.len() {
            scores.push(0);
        }
        for sec in 1..seconds {
            let mut cur_score = Vec::new();
            for r_id in 0..data.len() {
                let input = data[r_id].split(" ").collect::<Vec<&str>>();
                let speed: usize = input[3].parse().unwrap();
                let go_time: usize = input[6].parse().unwrap();
                let rest_time: usize = input[13].parse().unwrap();
 
                let loop_time = go_time + rest_time;
                let full_loops = sec / loop_time;
                let rem_loops = sec % loop_time;
                let distance;
                if rem_loops >= go_time {
                    distance = go_time * speed * (full_loops + 1);
                }
                else {
                    distance = (go_time * full_loops + rem_loops) * speed;
                }
                cur_score.push(distance);
            }
            let mut m_d = 0;
            for sc in &cur_score {
                if m_d < *sc {
                    m_d = *sc;
                }
            }
            for i in 0..cur_score.len() {
                if m_d == cur_score[i] {
                    scores[i] += 1;
                }
            }
        }
        for sc in &scores {
            if max_dist < *sc {
                max_dist = *sc;
            }
        }
    }
   println!("{}", max_dist);
    Ok(())
}
