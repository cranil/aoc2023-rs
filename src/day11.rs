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

fn solve(grid: &Grid<char>, ratio: i64) -> i64 {
    let mut galaxies = Vec::new();
    for y in 0..grid.height {
        for x in 0..grid.width {
            if grid.at(x, y).unwrap() == &'#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut empty_x = Vec::new();
    let mut empty_y = Vec::new();

    for x in 0..grid.width {
        let mut p = false;
        for y in 0..grid.height {
            if grid.at(x, y).unwrap() == &'#' {
                p = true;
            }
        }
        if !p {
            empty_x.push(x);
        }
    }

    for y in 0..grid.height {
        let mut p = false;
        for x in 0..grid.width {
            if grid.at(x, y).unwrap() == &'#' {
                p = true;
            }
        }
        if !p {
            empty_y.push(y);
        }
    }

    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in 0..i {
            let (x1, y1) = (galaxies[i].0 as i64, galaxies[i].1 as i64);
            let (x2, y2) = (galaxies[j].0 as i64, galaxies[j].1 as i64);
            let dx = x2 - x1;
            let dy = y2 - y1;
            let mut dist = (dx.abs() + dy.abs()) as i64;
            let min_x = std::cmp::min(x1, x2);
            let max_x = std::cmp::max(x1, x2);
            for x in empty_x.iter() {
                if *x as i64 > min_x && (max_x > *x as i64) {
                    dist += ratio - 1;
                }
            }

            let min_y = std::cmp::min(y1, y2);
            let max_y = std::cmp::max(y1, y2);
            for y in empty_y.iter() {
                if *y as i64 > min_y && (max_y > *y as i64) {
                    dist += ratio - 1;
                }
            }
            sum += dist;
        }
    }

    return sum;
}

fn part1(grid: &Grid<char>) -> i64 {
    return solve(grid, 2);
}

fn part2(grid: &Grid<char>) -> i64 {
    return solve(grid, 1_000_000);
}

test!(
    part1 {
        "test_inputs/day11/test01.txt" => 374i64
    },
    part2 {
        "test_inputs/day11/test01.txt" => 82000210i64
    }
);
main!();
