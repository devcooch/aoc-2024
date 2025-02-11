fn main() {
    let data = include_str!("day06.txt");
    let mut map: Vec<Vec<char>> = data
        .lines()
        .map(|x| x.chars().map(|y| y).collect())
        .collect();
    let n = map[0].len() as i32;
    let m = map.len() as i32;
    let dirs = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    let mut x: i32 = -1;
    let mut y: i32 = -1;
    for j in 0..map.len() {
        for i in 0..map[j].len() {
            if map[j][i] == '^' {
                x = i as i32;
                y = j as i32;
                break;
            }
        }
        if x >= 0 {
            break;
        }
    }
    let mut dir = 0;
    loop {
        map[y as usize][x as usize] = 'X';
        let (dx, dy) = dirs[dir];
        let new_x = x + dx;
        let new_y = y + dy;
        if new_x < 0 || new_y < 0 || new_x >= n || new_y >= m {
            break;
        }
        if map[new_y as usize][new_x as usize] == '#' {
            dir = (dir + 1) % 4;
            continue;
        }
        x = new_x;
        y = new_y;
    }
    println!(
        "{}",
        map.iter()
            .map(|row| row.iter().map(|c| (*c == 'X') as usize).sum::<usize>())
            .sum::<usize>()
    );
}
