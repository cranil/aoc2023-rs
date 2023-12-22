use std::{collections::HashSet, mem::swap};

use crate::{
    grid::Grid,
    utils::{main, read_lines, test},
};

type InputData = (Grid<char>, (usize, usize));

fn get_contents(filename: &str) -> InputData {
    let lines = read_lines(filename);
    let mut start = (0, 0);
    let mut grid = Grid::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
            if c == 'S' {
                start = (x, y);
                grid.set(x, y, '.');
            }
        }
    }
    return (grid, start);
}

fn get_neighbours(x: isize, y: isize) -> [(isize, isize); 4] {
    [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

fn part1((grid, start): &InputData) -> i64 {
    let start = (start.0 as isize, start.1 as isize);
    let mut frontier = HashSet::<(isize, isize)>::new();
    let mut expanded = HashSet::<(isize, isize)>::new();
    let steps = 50;
    expanded.insert(start.clone());
    for _ in 0..steps {
        let mut new_expanded = HashSet::new();
        frontier.extend(expanded.iter());
        for pos in expanded.iter() {
            let neighbours = get_neighbours(pos.0 as isize, pos.1 as isize);
            for neighbour in neighbours.iter() {
                if expanded.contains(neighbour) {
                    continue;
                }
                let Some(c) = grid.at(neighbour.0 as usize, neighbour.1 as usize) else {
                    continue;
                };
                if *c == '.' {
                    new_expanded.insert(neighbour.clone());
                }
            }
        }
        expanded = new_expanded;
    }
    expanded.len() as i64
}

fn part2((grid, start): &InputData) -> i64 {
    assert!(grid.width == grid.height);

    let start = (start.0 as isize, start.1 as isize);
    let mut curr = HashSet::new();
    let mut next = HashSet::new();
    let mut steps = 0;

    let mut expansion1 = 0;
    let mut expansion2 = 0;
    let mut _expansion3 = 0;

    let mut steps1 = 0;
    let mut steps2 = 0;

    curr.insert(start.clone());
    loop {
        // let mut next = HashSet::new();
        for pos in curr.iter() {
            let neighbours = get_neighbours(pos.0 as isize, pos.1 as isize);
            for neighbour in neighbours.iter() {
                let mut x = neighbour.0 + start.0;
                let mut y = neighbour.1 + start.1;
                if curr.contains(neighbour) {
                    continue;
                }

                if x < 0 {
                    let m = x / grid.width as isize;
                    x += (-m + 1) * grid.width as isize;
                }
                if y < 0 {
                    let m = y / grid.height as isize;
                    y += (-m + 1) * grid.height as isize;
                }
                x %= grid.width as isize;
                y %= grid.height as isize;

                if let Some(c) = grid.at(x as usize, y as usize) {
                    if *c == '.' {
                        next.insert(neighbour.clone());
                    }
                } else {
                    println!("{} {}", x, y);
                    panic!();
                }
            }
        }
        swap(&mut curr, &mut next);
        next.clear();
        steps += 1;

        let mut max_x = 0;
        let mut max_y = 0;
        let mut min_x = 0;
        let mut min_y = 0;

        for pos in curr.iter() {
            max_x = max_x.max(pos.0);
            max_y = max_y.max(pos.1);
            min_x = min_x.min(pos.0);
            min_y = min_y.min(pos.1);
        }

        let w = (max_x - min_x + 1) as usize;
        let h = (max_y - min_y + 1) as usize;

        if w == 3 * grid.width && h == 3 * grid.height {
            expansion1 = curr.len() as i64;
            steps1 = steps;
        }
        if w == 5 * grid.width && h == 5 * grid.height {
            expansion2 = curr.len() as i64;
            steps2 = steps;
        }
        if w == 7 * grid.width && h == 7 * grid.height {
            _expansion3 = curr.len() as i64;
            break;
        }
    }

    // only works for input.txt from the problem :(
    let a = (_expansion3 - 2 * expansion2 + expansion1) / 2;
    let b = (expansion2 - expansion1) - 3 * a;
    let c = expansion1 - a - b;

    let steps = 26501365;

    let m = steps2 - steps1;
    let n = steps1 - m;

    let k = (steps - n) / m;
    return a * k * k + b * k + c;
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
