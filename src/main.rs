use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn day1_read_input(filename: &str, a: &mut Vec<i32>, b: &mut Vec<i32>) {
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let numbers: Vec<i32> = line
            .unwrap()
            .split("   ")
            .map(|s| s.parse().unwrap())
            .collect();
        a.push(numbers[0]);
        b.push(numbers[1]);
    }
}

fn day1(filename: &str) {
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];
    day1_read_input(filename, &mut a, &mut b);
    a.sort();
    b.sort();
    for i in 0..100 {
        println!("{} {}", a[i], b[i]);
    }
    let mut sum: i32 = 0;
    for i in 0..a.len() {
        sum += (a[i] - b[i]).abs();
    }
    println!("{}", sum);
}

fn day1_followup(filename: &str) {
    let mut a: Vec<i32> = vec![];
    let mut b: Vec<i32> = vec![];
    day1_read_input(filename, &mut a, &mut b);
    let mut counter: HashMap<i32, i32> = HashMap::new();
    for n in b {
        counter.insert(n, counter.get(&n).copied().unwrap_or(0) + 1);
    }
    let mut sum = 0;
    for n in a {
        sum += counter.get(&n).copied().unwrap_or(0) * n;
    }
    println!("{}", sum);
}

fn day2_read_input(filename: &str) -> Vec<Vec<i32>> {
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);
    let numbers: Vec<Vec<i32>> = reader
        .lines()
        .map(|line| {
            line.unwrap()
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect();
    numbers
}

fn day2_unsafe_gap(a: i32, b: i32, increasing: bool) -> bool {
    return a == b || (a - b).abs() > 3 || (a > b) != increasing;
}

fn day2_unsafe_report(a: Vec<i32>, dampener_available: bool) -> bool {
    let mut increasing = a[1] > a[0];
    for i in 1..a.len() {
        if day2_unsafe_gap(a[i], a[i - 1], increasing) {
            if dampener_available {
                return ((if i > 1 {
                    // Deal with cases like this 8 5 7 9 11 13 where the target to remove is not in the offending gap.
                    day2_unsafe_report([&a[..i - 2], &a[i - 1..]].concat(), false)
                } else {
                    true
                }) && day2_unsafe_report([&a[..i - 1], &a[i..]].concat(), false)
                    && day2_unsafe_report([&a[..i], &a[i + 1..]].concat(), false));
            }
            return true;
        }
    }
    return false;
}

fn day2(filename: &str, enable_dampener: bool) {
    let reports = day2_read_input(filename);
    let mut sum: usize = 0;
    for report in reports {
        if !day2_unsafe_report(report, enable_dampener) {
            sum += 1;
        }
    }
    println!("{}", sum);
}

fn main() {
    day2("data/day2.txt", true);
}
