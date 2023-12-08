use std::collections::HashMap;

use crate::algos::Grid;
use crate::utils::{main, test};

fn get_contents(filename: &str) -> Grid<char> {
    let lines = crate::utils::read_lines(filename);
    let height = lines.len();
    let width = lines[0].len();
    let mut grid = Grid::new(width, height);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
        }
    }
    return grid;
}

fn is_symbol(c: &char) -> bool {
    match c {
        '0'..='9' => false,
        '.' => false,
        _ => true,
    }
}

fn is_gear(c: &char) -> bool {
    return *c == '*';
}

fn part1(grid: &Grid<char>) -> i32 {
    let mut sum = 0;

    for y in 0..grid.height {
        let mut num = 0;
        let mut adjecent = false;
        for x in 0..grid.width {
            let c = grid.at(x, y).unwrap();
            match c {
                '0'..='9' => {
                    num *= 10;
                    num += *c as i32 - '0' as i32;
                    if !adjecent {
                        if x > 0 && is_symbol(grid.at(x - 1, y).unwrap()) {
                            adjecent = true;
                        }
                        if x < grid.width - 1 && is_symbol(grid.at(x + 1, y).unwrap()) {
                            adjecent = true;
                        }
                        if y > 0 && is_symbol(grid.at(x, y - 1).unwrap()) {
                            adjecent = true;
                        }
                        if y < grid.height - 1 && is_symbol(grid.at(x, y + 1).unwrap()) {
                            adjecent = true;
                        }
                        if x > 0 && y > 0 && is_symbol(grid.at(x - 1, y - 1).unwrap()) {
                            adjecent = true;
                        }
                        if x < grid.width - 1 && y > 0 && is_symbol(grid.at(x + 1, y - 1).unwrap())
                        {
                            adjecent = true;
                        }
                        if x > 0 && y < grid.height - 1 && is_symbol(grid.at(x - 1, y + 1).unwrap())
                        {
                            adjecent = true;
                        }
                        if x < grid.width - 1
                            && y < grid.height - 1
                            && is_symbol(grid.at(x + 1, y + 1).unwrap())
                        {
                            adjecent = true;
                        }
                    }
                }
                _ => {
                    if num > 0 && adjecent {
                        sum += num;
                    }
                    adjecent = false;
                    num = 0;
                }
            }
        }
        if num > 0 && adjecent {
            sum += num;
        }
    }
    return sum;
}

fn part2(grid: &Grid<char>) -> i32 {
    let mut map = HashMap::<(usize, usize), Vec<i32>>::new();

    for y in 0..grid.height {
        let mut num = 0;
        let mut adjecent = false;
        let mut loc = (grid.width, grid.height);
        for x in 0..grid.width {
            let c = grid.at(x, y).unwrap();
            match c {
                '0'..='9' => {
                    num *= 10;
                    num += *c as i32 - '0' as i32;
                    if !adjecent {
                        if x > 0 && is_gear(grid.at(x - 1, y).unwrap()) {
                            adjecent = true;
                            loc = (x - 1, y);
                        }
                        if x < grid.width - 1 && is_gear(grid.at(x + 1, y).unwrap()) {
                            adjecent = true;
                            loc = (x + 1, y);
                        }
                        if y > 0 && is_gear(grid.at(x, y - 1).unwrap()) {
                            adjecent = true;
                            loc = (x, y - 1);
                        }
                        if y < grid.height - 1 && is_gear(grid.at(x, y + 1).unwrap()) {
                            adjecent = true;
                            loc = (x, y + 1);
                        }
                        if x > 0 && y > 0 && is_gear(grid.at(x - 1, y - 1).unwrap()) {
                            adjecent = true;
                            loc = (x - 1, y - 1);
                        }
                        if x < grid.width - 1 && y > 0 && is_gear(grid.at(x + 1, y - 1).unwrap()) {
                            adjecent = true;
                            loc = (x + 1, y - 1);
                        }
                        if x > 0 && y < grid.height - 1 && is_gear(grid.at(x - 1, y + 1).unwrap()) {
                            adjecent = true;
                            loc = (x - 1, y + 1);
                        }
                        if x < grid.width - 1
                            && y < grid.height - 1
                            && is_gear(grid.at(x + 1, y + 1).unwrap())
                        {
                            adjecent = true;
                            loc = (x + 1, y + 1);
                        }
                    }
                }
                _ => {
                    if num > 0 && adjecent {
                        map.entry(loc)
                            .and_modify(|v| v.push(num))
                            .or_insert(vec![num]);
                    }
                    adjecent = false;
                    num = 0;
                }
            }
        }
        if num > 0 && adjecent {
            map.entry(loc)
                .and_modify(|v| v.push(num))
                .or_insert(vec![num]);
        }
    }

    let sum = map
        .values()
        .filter(|v| v.len() == 2)
        .fold(0, |acc, v| acc + v[0] * v[1]);
    return sum;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i32); 1] = [("test_inputs/day03/test01.txt", 4361)];
    pub const PART2_INPUTS: [(&str, i32); 1] = [("test_inputs/day03/test01.txt", 467835)];
}

test!();
main!();
