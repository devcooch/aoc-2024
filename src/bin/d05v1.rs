fn main() {
    let mut lines = include_str!("day05.txt").lines();
    lines
        .take_while(|line| !line.is_empty())
        .map(|line| line.split('|').map(|x| x.parse::<usize>()));
}
