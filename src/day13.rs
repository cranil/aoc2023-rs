use crate::{
    grid::Grid,
    utils::{main, test},
};

fn get_contents(filename: &str) -> Vec<(Grid<char>, Grid<char>)> {
    let lines = crate::utils::read_lines(filename);
    return lines
        .split(|line| line.is_empty())
        .map(|lines| {
            let height = lines.len();
            let width = lines[0].len();
            let mut gridh = Grid::new(width, height);
            let mut gridv = Grid::new(height, width);
            for (y, line) in lines.iter().enumerate() {
                for (x, c) in line.chars().enumerate() {
                    gridh.set(x, y, c);
                    gridv.set(y, x, c);
                }
            }
            return (gridh, gridv);
        })
        .collect::<Vec<_>>();
}

fn reflection_count(grid: &Grid<char>) -> i64 {
    for y in 0..grid.height - 1 {
        let mut flag = true;
        for (y0, y1) in (0..=y).rev().zip(y + 1..grid.height) {
            for x in 0..grid.width {
                if grid.at(x, y0).unwrap() != grid.at(x, y1).unwrap() {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }

        if flag {
            return y as i64 + 1;
        }
    }
    return 0;
}

fn reflection_count_smudge(grid: &Grid<char>) -> i64 {
    for y in 0..grid.height - 1 {
        let mut flag = true;
        let mut smudge_wiped = false;
        for (y0, y1) in (0..=y).rev().zip(y + 1..grid.height) {
            for x in 0..grid.width {
                if grid.at(x, y0).unwrap() != grid.at(x, y1).unwrap() {
                    if smudge_wiped {
                        flag = false;
                        break;
                    }
                    smudge_wiped = true;
                }
            }
            if !flag {
                break;
            }
        }

        if flag && smudge_wiped {
            return y as i64 + 1;
        }
    }
    return 0;
}

fn part1(grids: &Vec<(Grid<char>, Grid<char>)>) -> i64 {
    let mut sum = 0;
    for (gridh, gridv) in grids {
        let counth = reflection_count(gridh);
        let countv = reflection_count(gridv);
        sum += 100 * counth + countv;
    }
    return sum as i64;
}

fn part2(grids: &Vec<(Grid<char>, Grid<char>)>) -> i64 {
    let mut sum = 0;
    for (gridh, gridv) in grids {
        let counth = reflection_count_smudge(gridh);
        let countv = reflection_count_smudge(gridv);
        println!("{} {}", counth, countv);
        sum += 100 * counth + countv;
    }
    return sum as i64;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
