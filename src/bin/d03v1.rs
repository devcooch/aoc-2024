use once_cell::sync::Lazy;
use regex::Regex;

fn main() {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());
    println!(
        "{}",
        include_str!("day03.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|l| RE
                .captures_iter(l)
                .map(|c| c
                    .iter()
                    .skip(1)
                    .filter_map(|m| m.map(|x| x.as_str().parse::<u64>().unwrap()))
                    .product::<u64>())
                .sum::<u64>())
            .sum::<u64>()
    );
}
