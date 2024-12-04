fn main() {
    let map: Vec<Vec<_>> = include_str!("day04.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let n = map.len();
    let m = map[0].len();
    let dxys: [(i32, i32); 2] = [(1, 1), (1, -1)];
    let mut total = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if map[i][j] == 'A' {
                let mut mas = 0;
                for (dx, dy) in dxys {
                    let x1 = (j as i32 + dx) as usize;
                    let y1 = (i as i32 + dy) as usize;
                    let x2 = (j as i32 - dx) as usize;
                    let y2 = (i as i32 - dy) as usize;
                    if map[y1][x1] == 'M' && map[y2][x2] == 'S'
                        || map[y1][x1] == 'S' && map[y2][x2] == 'M'
                    {
                        mas += 1;
                    }
                }
                if mas == 2 {
                    total += 1;
                }
            }
        }
    }
    println!("{}", total);
}
