use itertools::Itertools;

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
    let mut v2 = p.iter().map(|x| x.1).collect::<Vec<_>>();
    v1.sort();
    v2.sort();
    println!(
        "{}",
        v1.iter()
            .zip_eq(v2)
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
    );
}
