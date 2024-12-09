use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("src/day1/input.txt");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(s) => s,
    };

    let lines = s.lines();

    let mut left_list = Vec::<i32>::new();
    let mut right_list = Vec::<i32>::new();

    for l in lines {
        let toks: Vec<&str> = l.split("   ").collect();
        if toks.len() == 2 {
            left_list.push(toks[0].parse::<i32>().unwrap());
            right_list.push(toks[1].parse::<i32>().unwrap());
        }
    }
    left_list.sort();
    right_list.sort();

    let mut sum: i64 = 0;

    for i in 0..left_list.len() {
        let left = left_list[i];
        let right = right_list[i];
        sum += (left - right).abs() as i64;
    }
    println!("Part 1 solution: {:?}", sum);

    let mut right_count = HashMap::<i32, i32>::new();
    for r in right_list {
        let count = right_count.entry(r).or_insert(0);
        *count += 1;
    }

    let mut similarity_sum: i64 = 0;

    for l in left_list {
        similarity_sum += l as i64 * *right_count.get(&l).unwrap_or(&0) as i64;
    }

    println!("Part 2 solution: {:?}", similarity_sum);
}
