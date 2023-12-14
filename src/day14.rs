use crate::{
    grid::Grid,
    utils::{main, test},
};

fn get_contents(filename: &str) -> Grid<char> {
    let lines = crate::utils::read_lines(filename);
    let mut grid = Grid::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
        }
    }
    return grid;
}

fn calc_load(grid: &Grid<char>) -> i64 {
    let mut sum = 0;
    for x in 0..grid.width {
        for y in 0..grid.height {
            match grid.at(x, y).unwrap() {
                'O' => {
                    sum += (grid.height - y) as i64;
                }
                _ => {}
            }
        }
    }
    return sum;
}

fn north(grid: &mut Grid<char>) {
    for x in 0..grid.width {
        let mut yp = 0;
        for y in 0..grid.height {
            match grid.at(x, y).unwrap() {
                'O' => {
                    if yp != y {
                        grid.set(x, yp, 'O');
                        grid.set(x, y, '.');
                    }
                    yp += 1;
                }
                '#' => {
                    yp = y + 1;
                }
                _ => {}
            }
        }
    }
}

fn west(grid: &mut Grid<char>) {
    for y in 0..grid.height {
        let mut xp = 0;
        for x in 0..grid.width {
            match grid.at(x, y).unwrap() {
                'O' => {
                    if x != xp {
                        grid.set(xp, y, 'O');
                        grid.set(x, y, '.');
                    }
                    xp += 1;
                }
                '#' => {
                    xp = x + 1;
                }
                _ => {}
            }
        }
    }
}

fn south(grid: &mut Grid<char>) {
    for x in 0..grid.width {
        let mut yp = grid.height - 1;
        for y in (0..grid.height).rev() {
            match grid.at(x, y).unwrap() {
                'O' => {
                    if y != yp {
                        grid.set(x, yp, 'O');
                        grid.set(x, y, '.');
                    }
                    yp -= 1;
                }
                '#' => {
                    yp = y - 1;
                }
                _ => {}
            }
        }
    }
}

fn east(grid: &mut Grid<char>) {
    for y in 0..grid.height {
        let mut xp = grid.width - 1;
        for x in (0..grid.width).rev() {
            match grid.at(x, y).unwrap() {
                'O' => {
                    if xp != x {
                        grid.set(xp, y, 'O');
                        grid.set(x, y, '.');
                    }
                    xp -= 1;
                }
                '#' => {
                    xp = x - 1;
                }
                _ => {}
            }
        }
    }
}

fn cycle(grid: &mut Grid<char>) {
    north(grid);
    west(grid);
    south(grid);
    east(grid);
}

fn part1(grid: &Grid<char>) -> i64 {
    let mut grid = grid.clone();
    north(&mut grid);
    return calc_load(&grid);
}

fn grid_eq(grid1: &Grid<char>, grid2: &Grid<char>) -> bool {
    for x in 0..grid1.width {
        for y in 0..grid1.height {
            if grid1.at(x, y).unwrap() != grid2.at(x, y).unwrap() {
                return false;
            }
        }
    }
    return true;
}

fn part2(grid: &Grid<char>) -> i64 {
    let mut grid = grid.clone();
    let mut prev = Vec::new();
    let mut n_runs = 0;
    for l in 0..1_000 {
        cycle(&mut grid);
        if let Some(i) = prev.iter().position(|g| grid_eq(g, &grid)) {
            n_runs = l;
            for _ in 0..i {
                prev.remove(0);
            }
            break;
        }
        prev.push(grid.clone());
    }
    return calc_load(&prev[(1_000_000_000 - n_runs - 1) % prev.len()]);
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
