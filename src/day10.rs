use std::collections::{HashMap, VecDeque};

use crate::{
    grid::Grid,
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

fn cmap(c: &char) -> char {
    let p = match c {
        'S' => '*',
        'L' => '╚',
        'F' => '╔',
        '7' => '╗',
        'J' => '╝',
        '|' => '║',
        '-' => '═',
        '.' => '■',
        _ => ' ',
    };
    return p;
}

#[allow(dead_code)]
fn pretty_print_cycle(
    grid: &Grid<char>,
    cycle: &Vec<(usize, usize)>,
    interior_points: &Vec<(usize, usize)>,
) {
    for _ in 0..grid.width + 2 {
        print!("█");
    }
    println!("");
    for y in 0..grid.height {
        print!("█");
        for x in 0..grid.width {
            let c = grid.at(x, y).unwrap();
            let p = cmap(c);
            if cycle.contains(&(x, y)) {
                print!("\x1b[0;36m{}\x1b[0m", p);
            } else {
                if interior_points.contains(&(x, y)) {
                    print!("\x1b[0;32m{}\x1b[0m", p);
                } else {
                    print!("\x1b[0;31m{}\x1b[0m", p);
                }
            }
        }
        print!("█");
        println!("");
    }
    for _ in 0..grid.width + 2 {
        print!("█");
    }
    println!("");
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
        let mut debug = Grid::new(grid.width, grid.height);
        for x in 0..grid.width {
            for y in 0..grid.height {
                debug.set(x, y, '.');
            }
        }
        debug.set(start.0, start.1, '*');
        let mut visited = Grid::new(grid.width, grid.height);
        let (nx, ny) = (start.0 as isize + di, start.1 as isize + dj);
        if nx < 0 || ny < 0 || nx >= grid.width as isize || ny >= grid.height as isize {
            continue;
        }
        let (ni, nj) = (nx as usize, ny as usize);
        let mut stack = VecDeque::new();
        let mut cycle = Vec::new();
        stack.push_front((ni, nj));
        // dfs
        while !stack.is_empty() {
            let (x, y) = stack.pop_front().unwrap();
            if *visited.at(x, y).unwrap() {
                continue;
            }
            visited.set(x, y, true);

            let c = grid.at(x, y).unwrap();
            match c {
                'S' => {
                    cycle.push((x, y));
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
            debug.set(x, y, cmap(c));
            // println!("{}", debug);
            cycle.push((x, y));
        }
        if cycle.len() > 1 {
            if is_neighbor(cycle[0], cycle[cycle.len() - 1]) {
                cycles.push(cycle);
            }
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

fn is_neighbor(p0: (usize, usize), p1: (usize, usize)) -> bool {
    let (x0, y0) = p0;
    let (x1, y1) = p1;
    if x0 == x1 {
        return y0 == y1 + 1 || y0 == y1 - 1;
    }
    if y0 == y1 {
        return x0 == x1 + 1 || x0 == x1 - 1;
    }
    return false;
}

fn find_interior_points(grid: &Grid<char>, cycle: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut interior_points = Vec::new();
    for y in 0..grid.height {
        let mut cycle_y = cycle.iter().filter(|(_, yp)| y == *yp).collect::<Vec<_>>();
        cycle_y.sort_by(|a, b| a.0.cmp(&b.0));

        for x in 0..grid.width {
            let mut count0 = 0;

            if *grid.at(x, y).unwrap() != '.' {
                continue;
            }
            cycle_y
                .iter()
                .filter(|(xp, _)| *xp >= x)
                .for_each(|(xp, _)| {
                    match grid.at(*xp, y).unwrap() {
                        // count saddles as interections (eg. F---J, L---7) and ignore tangents (eg. L---J, F---7)
                        '7' | 'F' | '-' => {}
                        _ => {
                            count0 += 1;
                        }
                    }
                });
            if count0 % 2 == 1 {
                interior_points.push((x, y));
            }
        }
    }
    return interior_points;
}

fn part2(grid: &(Grid<char>, (usize, usize))) -> i64 {
    let (grid_o, start) = grid;
    let mut cycles = find_cycle(&grid_o, start);
    cycles.sort_by(|a, b| a.len().cmp(&b.len()));
    let cycle = cycles.last().unwrap();

    let mut grid = Grid::new(grid_o.width, grid_o.height);
    for x in 0..grid.width {
        for y in 0..grid.height {
            grid.set(x, y, '.');
        }
    }

    for (x, y) in cycle {
        grid.set(*x, *y, *grid_o.at(*x, *y).unwrap());
    }

    let (x0, y0) = start;
    let (x1, y1) = cycle[0];
    let (dx0, dy0) = (x1 as isize - *x0 as isize, y1 as isize - *y0 as isize);
    let (x1, y1) = cycle[cycle.len() - 2];
    let (dx1, dy1) = (x1 as isize - *x0 as isize, y1 as isize - *y0 as isize);

    match (dx0, dy0, dx1, dy1) {
        (0, 1, 1, 0) | (1, 0, 0, 1) => {
            grid.set(start.0, start.1, 'F');
        }
        (0, -1, 1, 0) | (1, 0, 0, -1) => {
            grid.set(start.0, start.1, 'L');
        }
        (0, 1, -1, 0) | (-1, 0, 0, 1) => {
            grid.set(start.0, start.1, 'J');
        }
        (0, -1, -1, 0) | (-1, 0, 0, -1) => {
            grid.set(start.0, start.1, '7');
        }
        (1, 0, -1, 0) | (-1, 0, 1, 0) => {
            grid.set(start.0, start.1, '-');
        }
        (0, 1, 0, -1) | (0, -1, 0, 1) => {
            grid.set(start.0, start.1, '|');
        }
        _ => {
            panic!("Invalid cycle");
        }
    }

    let interior_points = find_interior_points(&grid, cycle);
    // pretty_print_cycle(&grid_o, &cycle, &interior_points);
    return interior_points.len() as i64;
}

test!(
    part1 {
        "test_inputs/day10/test01.txt" => 8
    },
    part2 {
        "test_inputs/day10/test02.txt" => 8
    }
);
main!();
