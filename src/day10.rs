use std::collections::{HashMap, VecDeque};

use crate::{
    algos::Grid,
    utils::{main, test},
};

fn get_contents(filename: &str) -> (Grid<char>, (usize, usize)) {
    let lines = crate::utils::read_lines(filename);
    let mut grid = Grid::new(lines[0].len(), lines.len());
    let mut start = (0, 0);
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
            if c == 'S' {
                start = (x, y);
            }
        }
    }
    return (grid, start);
}

fn pretty_print_cycle(
    grid: &Grid<char>,
    cycle: &Vec<(usize, usize)>,
    interior_points: &Vec<(usize, usize)>,
) {
    let mut pretty_grid = Grid::new(grid.width + 2, grid.height + 2);
    for x in 0..grid.width {
        for y in 0..grid.height {
            let c = grid.at(x, y).unwrap();
            if cycle.contains(&(x, y)) {
                let p = match c {
                    'S' => '*',
                    'L' => '└',
                    'F' => '┌',
                    '7' => '┐',
                    'J' => '┘',
                    '|' => '│',
                    '-' => '─',
                    '.' => '.',
                    _ => ' ',
                };
                pretty_grid.set(x + 1, y + 1, p);
            } else {
                pretty_grid.set(x + 1, y + 1, '.');
            }
            if interior_points.contains(&(x, y)) {
                pretty_grid.set(x + 1, y + 1, '█');
            }
        }
    }
    for x in 0..grid.width + 2 {
        pretty_grid.set(x, 0, '█');
        pretty_grid.set(x, grid.height + 1, '█');
    }
    for y in 0..grid.height + 2 {
        pretty_grid.set(0, y, '█');
        pretty_grid.set(grid.width + 1, y, '█');
    }

    println!("{}", pretty_grid);
}

fn find_cycle(grid: &Grid<char>, start: &(usize, usize)) -> Vec<Vec<(usize, usize)>> {
    let mut connections_h = HashMap::new();
    let mut connections_v = HashMap::new();
    connections_h.insert('L', vec!['S', '7', 'J', '-']);
    connections_h.insert('F', vec!['S', '7', 'J', '-']);
    connections_h.insert('7', vec!['S', 'L', 'F', '-']);
    connections_h.insert('J', vec!['S', 'L', 'F', '-']);
    connections_h.insert('-', vec!['S', 'L', 'F', '7', 'J', '-']);
    connections_h.insert('S', vec!['L', 'F', '7', 'J', '-']);

    connections_v.insert('L', vec!['S', '7', 'F', '|']);
    connections_v.insert('F', vec!['S', 'J', 'L', '|']);
    connections_v.insert('7', vec!['S', 'J', 'L', '|']);
    connections_v.insert('J', vec!['S', 'F', '7', '|']);
    connections_v.insert('|', vec!['S', 'L', 'F', '7', 'J', '|']);
    connections_v.insert('S', vec!['L', 'F', '7', 'J', '|']);

    let mut cycles = Vec::new();
    for (di, dj) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
        let mut visited = Grid::new(grid.width, grid.height);
        visited.set(start.0, start.1, true);

        let (ni, nj) = (start.0 as isize + di, start.1 as isize + dj);
        if ni < 0 || nj < 0 || ni >= grid.height as isize || nj >= grid.width as isize {
            continue;
        }
        let (ni, nj) = (ni as usize, nj as usize);
        let mut stack = VecDeque::new();
        let mut cycle = Vec::new();
        cycle.push((start.0, start.1));
        stack.push_front((ni, nj));
        // dfs
        while !stack.is_empty() {
            let (x, y) = stack.pop_front().unwrap();
            if *visited.at(x, y).unwrap() {
                continue;
            }
            visited.set(x, y, true);
            match grid.at(x, y).unwrap() {
                'S' => {
                    break;
                }
                'L' => {
                    let right = grid.at(x + 1, y);
                    let up = grid.at(x, y - 1);
                    if let Some(right) = right {
                        if !connections_h.get(&'L').unwrap().contains(&right) {
                            break;
                        }
                        stack.push_front((x + 1, y));
                    }
                    if let Some(up) = up {
                        if !connections_v.get(&'L').unwrap().contains(&up) {
                            break;
                        }
                        stack.push_front((x, y - 1));
                    }
                }
                'F' => {
                    let right = grid.at(x + 1, y);
                    let down = grid.at(x, y + 1);
                    if let Some(right) = right {
                        if !connections_h.get(&'F').unwrap().contains(&right) {
                            println!("right: {:?}, {:?}", right, connections_h.get(&'F').unwrap());
                            break;
                        }
                        stack.push_front((x + 1, y));
                    }
                    if let Some(down) = down {
                        if !connections_v.get(&'F').unwrap().contains(&down) {
                            break;
                        }
                        stack.push_front((x, y + 1));
                    }
                }
                '7' => {
                    let left = grid.at(x - 1, y);
                    let down = grid.at(x, y + 1);
                    if let Some(left) = left {
                        if !connections_h.get(&'7').unwrap().contains(&left) {
                            break;
                        }
                        stack.push_front((x - 1, y));
                    }
                    if let Some(down) = down {
                        if !connections_v.get(&'7').unwrap().contains(&down) {
                            break;
                        }
                        stack.push_front((x, y + 1));
                    }
                }
                'J' => {
                    let left = grid.at(x - 1, y);
                    let up = grid.at(x, y - 1);
                    if let Some(left) = left {
                        if !connections_h.get(&'J').unwrap().contains(&left) {
                            break;
                        }
                        stack.push_front((x - 1, y));
                    }
                    if let Some(up) = up {
                        if !connections_v.get(&'J').unwrap().contains(&up) {
                            break;
                        }
                        stack.push_front((x, y - 1));
                    }
                }
                '|' => {
                    let down = grid.at(x, y + 1);
                    let up = grid.at(x, y - 1);
                    if let Some(right) = down {
                        if let Some(v) = connections_v.get(&right) {
                            if !v.contains(&'|') {
                                break;
                            }
                        } else {
                            break;
                        }
                        stack.push_front((x, y + 1));
                    }
                    if let Some(up) = up {
                        if let Some(v) = connections_v.get(&up) {
                            if !v.contains(&'|') {
                                break;
                            }
                        } else {
                            break;
                        }
                        stack.push_front((x, y - 1));
                    }
                }
                '-' => {
                    let left = grid.at(x - 1, y);
                    let right = grid.at(x + 1, y);
                    if let Some(left) = left {
                        if let Some(v) = connections_h.get(&left) {
                            if !v.contains(&'-') {
                                break;
                            }
                        } else {
                            break;
                        }
                        stack.push_front((x - 1, y));
                    }
                    if let Some(right) = right {
                        if let Some(v) = connections_h.get(&right) {
                            if !v.contains(&'-') {
                                break;
                            }
                        } else {
                            break;
                        }
                        stack.push_front((x + 1, y));
                    }
                }
                '.' => {
                    break;
                }
                _ => {
                    panic!("Invalid char");
                }
            }
            cycle.push((x, y));
        }
        if cycle.len() > 1 {
            cycles.push(cycle);
        }
    }
    return cycles;
}

fn part1(grid: &(Grid<char>, (usize, usize))) -> i64 {
    let (grid, start) = grid;
    let mut cycles = find_cycle(grid, start);
    cycles.sort_by(|a, b| a.len().cmp(&b.len()));
    let cycle = cycles.last().unwrap();
    return ((cycle.len() + 1) / 2) as i64;
}

fn find_interior_points(grid: &Grid<char>, cycle: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut interior_points = Vec::new();
    for x in 0..grid.width {
        for y in 0..grid.height {
            let mut count0 = 0;

            if cycle.contains(&(x, y)) {
                continue;
            }
            let mut xt = x as i64;
            while xt < grid.width as i64 {
                xt += 1;
                if cycle.contains(&(xt as usize, y)) {
                    match grid.at(xt as usize, y).unwrap() {
                        '7' | 'F' | '-' => {}
                        _ => {
                            count0 += 1;
                        }
                    }
                }
            }
            if count0 % 2 == 1 {
                interior_points.push((x, y));
            }
        }
    }
    return interior_points;
}

fn part2(grid: &(Grid<char>, (usize, usize))) -> i64 {
    let (grid, start) = grid;
    let mut cycles = find_cycle(&grid, start);
    cycles.sort_by(|a, b| a.len().cmp(&b.len()));
    let cycle = cycles.last().unwrap();
    let interior_points = find_interior_points(&grid, cycle);
    // pretty_print_cycle(&grid, &cycle, &interior_points);
    return interior_points.len() as i64;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 1] = [("test_inputs/day10/test01.txt", 8)];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
