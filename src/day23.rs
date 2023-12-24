use std::collections::{HashMap, HashSet};

use crate::{
    algos::priority_queue::BinaryHeap,
    grid,
    utils::{main, read_lines, test},
};

type InputData = grid::Grid<char>;

fn get_contents(filename: &str) -> InputData {
    let lines = read_lines(filename);
    let mut grid = grid::Grid::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
        }
    }
    return grid;
}

fn find_start(grid: &InputData) -> (usize, usize) {
    for x in 0..grid.width {
        if grid.at(x, 0).unwrap() == &'.' {
            return (x, 0);
        }
    }
    panic!("No start found");
}

fn find_end(grid: &InputData) -> (usize, usize) {
    for x in 0..grid.width {
        if grid.at(x, grid.height - 1).unwrap() == &'.' {
            return (x, grid.height - 1);
        }
    }
    panic!("No end found");
}

fn part1(grid: &InputData) -> i64 {
    let start = find_start(grid);
    let end = find_end(grid);
    let bound_check =
        |x: i64, y: i64| x >= 0 && x < grid.width as i64 && y >= 0 && y < grid.height as i64;

    let mut queue = BinaryHeap::new();
    let mut neg_max_length = grid::Grid::new(grid.width, grid.height);
    neg_max_length.fill(0);

    queue.push(start, -1);
    let mut edges = HashSet::new();

    while let Some(((x, y), _)) = queue.pop() {
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut neighbours = Vec::new();
        for (dx, dy) in dirs.iter() {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if !bound_check(nx, ny) || grid.at(nx as usize, ny as usize).unwrap() == &'#' {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if edges.contains(&((nx, ny), (x, y))) {
                continue;
            }
            let c = grid.at(nx, ny).unwrap();
            match (c, dx, dy) {
                ('>', 1, 0) => neighbours.push((nx, ny)),
                ('<', -1, 0) => neighbours.push((nx, ny)),
                ('^', 0, -1) => neighbours.push((nx, ny)),
                ('v', 0, 1) => neighbours.push((nx, ny)),
                ('.', _, _) => neighbours.push((nx, ny)),
                _ => continue,
            }
            edges.insert(((x, y), (nx, ny)));
        }
        for (nx, ny) in neighbours {
            let current_neg_length = *neg_max_length.at(x, y).unwrap();
            let new_neg_length = (current_neg_length - 1).min(*neg_max_length.at(nx, ny).unwrap());

            neg_max_length.set(nx, ny, new_neg_length);
            queue.push((nx, ny), *neg_max_length.at(nx, ny).unwrap());
        }
    }
    -*neg_max_length.at(end.0, end.1).unwrap()
}

fn dfs(
    grid: &InputData,
    edges: &HashMap<(usize, usize), HashSet<((usize, usize), i64)>>,
    start: (usize, usize),
    end: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    cost: i64,
) -> i64 {
    if start.1 == end.1 {
        return cost;
    }
    visited.insert(start);
    let res = edges[&start]
        .iter()
        .filter(|(n, _)| !visited.contains(n))
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(n, w)| dfs(grid, edges, n, end, visited, cost + w))
        .max()
        .unwrap_or_default();
    visited.remove(&start);
    res
}

fn part2(grid: &InputData) -> i64 {
    let start = find_start(grid);
    let end = find_end(grid);
    let bound_check =
        |x: i64, y: i64| x >= 0 && x < grid.width as i64 && y >= 0 && y < grid.height as i64;
    let get_neighbours = |x, y| {
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut neighbours = Vec::new();
        for (dx, dy) in dirs.iter() {
            let nx = x as i64 + dx;
            let ny = y as i64 + dy;
            if !bound_check(nx, ny) || grid.at(nx as usize, ny as usize).unwrap() == &'#' {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            neighbours.push((nx, ny));
        }
        neighbours
    };

    // Big graph
    let mut nodes = HashSet::new();
    let mut edges = HashMap::new();

    for x in 0..grid.width {
        for y in 0..grid.height {
            if grid.at(x, y).unwrap() == &'#' {
                continue;
            }
            nodes.insert((x, y));
            let neighbours = get_neighbours(x, y);
            for (nx, ny) in neighbours {
                edges
                    .entry((x, y))
                    .or_insert_with(HashSet::new)
                    .insert(((nx, ny), 1));
            }
        }
    }

    // small graph
    let mut new_nodes = nodes
        .iter()
        .filter(|(x, y)| edges.get(&(*x, *y)).unwrap().len() > 2)
        .map(|(x, y)| (*x, *y))
        .collect::<HashSet<_>>();

    new_nodes.insert(start);
    new_nodes.insert(end);

    let mut new_edges = HashMap::new();

    for n in new_nodes.iter() {
        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        stack.push((n.0, n.1, 0));
        while let Some((x, y, dist)) = stack.pop() {
            if ((x, y) != *n) && new_nodes.contains(&(x, y)) {
                new_edges
                    .entry(*n)
                    .or_insert_with(HashSet::new)
                    .insert(((x, y), dist));
                continue;
            }

            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));
            let neighbours = edges.get(&(x, y)).unwrap();
            for ((nx, ny), _) in neighbours {
                let nx = *nx;
                let ny = *ny;
                stack.push((nx, ny, dist + 1));
            }
        }
    }

    dfs(grid, &new_edges, start, end, &mut HashSet::new(), 0)
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
