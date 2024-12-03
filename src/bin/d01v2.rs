use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let p = include_str!("day01.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|l| {
            l.split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
        })
        .tuples::<(_, _)>()
        .collect::<Vec<_>>();
    let mut v1 = p.iter().map(|x| x.0).collect::<Vec<_>>();
    let v2 = p.iter().fold(HashMap::new(), |mut counts, x| {
        *counts.entry(x.1).or_insert(0i32) += 1;
        counts
    });
    v1.sort();
    println!(
        "{}",
        v1.iter()
            .map(|x| x * v2.get(x).or(Some(&0i32)).unwrap())
            .sum::<i32>()
    );
}
