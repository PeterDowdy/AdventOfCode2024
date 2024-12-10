use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("src/day2/input.txt");
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

    let mut report_list = Vec::<Vec<i32>>::new();

    for l in lines {
        let toks: Vec<&str> = l.split(' ').collect();
        let mut row = Vec::<i32>::new();
        for t in toks {
            row.push(t.parse::<i32>().unwrap());
        }
        report_list.push(row);
    }

    let mut safe_reports = Vec::<Vec<i32>>::new();

    for r in report_list.clone() {
        let mut all_ascending: bool = true;
        let mut all_descending: bool = true;
        let mut largest_gap: i32 = 0;
        let mut smallest_gap: i32 = i32::max_value();

        for i in 1..r.len() {
            if r[i] > r[i - 1] {
                all_descending = false;
            } else if r[i] < r[i - 1] {
                all_ascending = false;
            }

            let gap = (r[i] - r[i - 1]).abs();
            if gap > largest_gap {
                largest_gap = gap;
            }
            if gap < smallest_gap {
                smallest_gap = gap;
            }
        }

        if (all_ascending || all_descending) && smallest_gap >= 1 && largest_gap <= 3 {
            safe_reports.push(r.clone());
        }
    }

    println!("Day 1 result: {:?}", safe_reports.len());

    safe_reports = Vec::<Vec<i32>>::new();

    for r in report_list.clone() {
        let mut is_safe = false;

        for j in 0..r.len() {
            let mut sublist = Vec::<i32>::new();
            for k in 0..r.len() {
                if j != k {
                    sublist.push(r[k]);
                }
            }
            let mut all_ascending: bool = true;
            let mut all_descending: bool = true;
            let mut largest_gap: i32 = 0;
            let mut smallest_gap: i32 = i32::max_value();

            for i in 1..sublist.len() {
                if sublist[i] > sublist[i - 1] {
                    all_descending = false;
                } else if sublist[i] < sublist[i - 1] {
                    all_ascending = false;
                }

                let gap = (sublist[i] - sublist[i - 1]).abs();
                if gap > largest_gap {
                    largest_gap = gap;
                }
                if gap < smallest_gap {
                    smallest_gap = gap;
                }
            }

            is_safe = is_safe
                || (all_ascending || all_descending) && smallest_gap >= 1 && largest_gap <= 3;
        }
        if is_safe {
            safe_reports.push(r.clone());
        }
    }

    println!("Day 2 result: {:?}", safe_reports.len());
}
