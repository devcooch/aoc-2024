fn is_safe(v: &[i32]) -> bool {
    let d = v
        .iter()
        .zip(v.iter().skip(1))
        .map(|(x, y)| x - y)
        .collect::<Vec<_>>();
    d.iter().all(|&x| x.abs() < 4)
        && d.iter().map(|x| x.signum()).sum::<i32>().abs() == i32::try_from(d.len()).ok().unwrap()
}

fn main() {
    println!(
        "{}",
        include_str!("day02.txt")
            .lines()
            .filter(|line| !line.is_empty())
            .map(|l| l
                .split_ascii_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>())
            .filter(|v| is_safe(v))
            .count()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        assert_eq!(true, is_safe(&[7, 6, 4, 2, 1]));
        assert_eq!(false, is_safe(&[1, 2, 7, 8, 9]));
        assert_eq!(false, is_safe(&[9, 7, 6, 2, 1]));
        assert_eq!(false, is_safe(&[1, 3, 2, 4, 5]));
        assert_eq!(false, is_safe(&[8, 6, 4, 4, 1]));
        assert_eq!(true, is_safe(&[1, 3, 6, 7, 9]));
    }
}
