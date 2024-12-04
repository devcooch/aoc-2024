fn main() {
    let map: Vec<Vec<_>> = include_str!("day04.txt")
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    let n = map.len();
    let m = map[0].len();
    let dxys: [(i32, i32); 8] = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];
    let mut total = 0;
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 'X' {
                for (dx, dy) in dxys {
                    let x1 = j as i32 + dx;
                    let x2 = j as i32 + 2 * dx;
                    let x3 = j as i32 + 3 * dx;
                    if x3 < 0 || x3 >= m as i32 {
                        continue;
                    }
                    let y1 = i as i32 + dy;
                    let y2 = i as i32 + 2 * dy;
                    let y3 = i as i32 + 3 * dy;
                    if y3 < 0 || y3 >= n as i32 {
                        continue;
                    }
                    if map[y1 as usize][x1 as usize] == 'M'
                        && map[y2 as usize][x2 as usize] == 'A'
                        && map[y3 as usize][x3 as usize] == 'S'
                    {
                        total += 1;
                    }
                }
            }
        }
    }
    println!("{}", total);
}
