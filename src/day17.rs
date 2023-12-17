use crate::{
    algos::priority_queue,
    grid::Grid,
    utils::{main, read_lines, test},
};

fn get_contents(filename: &str) -> Grid<i64> {
    let lines = read_lines(filename);
    let mut grid = Grid::new(lines[0].len(), lines.len());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c.to_string().parse::<i64>().unwrap());
        }
    }
    return grid;
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
const LEFT: usize = 0;
const RIGHT: usize = 1;
const UP: usize = 2;
const DOWN: usize = 3;

const MAX_BLOCK_MOVES: usize = 16;

pub fn print_path(grid: &Grid<i64>, path: &[(usize, usize, usize, usize)]) {
    let mut path_grid = Grid::new(grid.width, grid.height);
    path_grid.fill('.');
    let mut total_cost = 0;
    for &(x, y, dir, _) in path {
        let c = match dir {
            LEFT => '<',
            RIGHT => '>',
            UP => '^',
            DOWN => 'v',
            _ => panic!("invalid direction"),
        };
        path_grid.set(x, y, c);
        total_cost += *grid.at(x, y).unwrap();
    }
    println!("{}", path_grid);
    println!("total cost: {}", total_cost);
}

type State = (usize, usize, usize, usize);

fn find_min(
    costs: &Grid<i64>,
    start: (usize, usize),
    get_neighbours: fn(State, usize, usize) -> Vec<State>,
) -> Vec<Vec<i64>> {
    let mut queue = priority_queue::BinaryHeap::new();
    let mut parent =
        vec![vec![vec![vec![None; MAX_BLOCK_MOVES]; DIRECTIONS.len()]; costs.height]; costs.width];
    let mut dists =
        vec![
            vec![vec![vec![std::i64::MAX / 2; MAX_BLOCK_MOVES]; DIRECTIONS.len()]; costs.height];
            costs.width
        ];

    for len in 0..MAX_BLOCK_MOVES {
        for dir_idx in 0..DIRECTIONS.len() {
            dists[start.0][start.1][dir_idx][len] = 0;
        }
    }
    queue.push((start.0, start.1, RIGHT, 0), 0);
    while !queue.is_empty() {
        let (state, _) = queue.pop().unwrap();
        let (x, y, dir_idx, blocks_moved) = state;
        let neighbours = get_neighbours(state, costs.width, costs.height);
        for neighbour in neighbours {
            let (nx, ny, ndir_idx, nblocks_moved) = neighbour;
            let cost = *costs.at(nx, ny).unwrap();
            let dist = dists[x][y][dir_idx][blocks_moved] + cost;
            if dist < dists[nx][ny][ndir_idx][nblocks_moved] {
                dists[nx][ny][ndir_idx][nblocks_moved] = dist;
                queue.push((nx, ny, ndir_idx, nblocks_moved), dist);
                parent[nx][ny][ndir_idx][nblocks_moved] = Some((x, y, dir_idx, blocks_moved));
            }
        }
    }

    // let mut min = i64::MAX;
    // let mut argmin = None;
    // for len in 4..MAX_LEN {
    //    for dir_idx in 0..DIRECTIONS.len() {
    //        let dist = dists[costs.width - 1][costs.height - 1][dir_idx][len];
    //        if dist < min {
    //            min = dist;
    // argmin = Some((costs.width - 1, costs.height - 1, dir_idx, len));
    //        }
    //    }
    // }
    // let mut path = Vec::new();
    // let mut state = argmin.unwrap();
    // while let Some(prev_state) = parent[state.0][state.1][state.2][state.3] {
    //     path.push(state);
    //     state = prev_state;
    // }
    return dists[costs.width - 1][costs.height - 1].clone();
}

fn get_neighbours(
    (x, y, dir_idx, blocks_moved): State,
    width: usize,
    height: usize,
    min_block_moves: usize,
    max_block_moves: usize,
) -> Vec<State> {
    let mut neighbours = Vec::new();
    let (dx, dy) = DIRECTIONS[dir_idx];
    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
    if blocks_moved == 0 {
        for dir in 0..DIRECTIONS.len() {
            let (dx, dy) = DIRECTIONS[dir];
            let (nx, ny) = (x as i64 + dx, y as i64 + dy);
            if nx >= 0 && nx < width as i64 && ny >= 0 && ny < height as i64 {
                neighbours.push((nx as usize, ny as usize, dir, 1));
            }
        }
    }
    if blocks_moved < max_block_moves
        && nx >= 0
        && nx < width as i64
        && ny >= 0
        && ny < height as i64
    {
        neighbours.push((nx as usize, ny as usize, dir_idx, blocks_moved + 1));
    }
    if blocks_moved < min_block_moves {
        return neighbours;
    } else {
        let ndir_idx = match dir_idx {
            LEFT => UP,
            RIGHT => UP,
            UP => RIGHT,
            DOWN => RIGHT,
            _ => panic!("invalid direction"),
        };
        let (dx, dy) = DIRECTIONS[ndir_idx];
        let (nx, ny) = (x as i64 + dx, y as i64 + dy);
        if blocks_moved <= max_block_moves
            && nx >= 0
            && nx < width as i64
            && ny >= 0
            && ny < height as i64
        {
            neighbours.push((nx as usize, ny as usize, ndir_idx, 1));
        }

        let ndir_idx = match dir_idx {
            LEFT => DOWN,
            RIGHT => DOWN,
            UP => LEFT,
            DOWN => LEFT,
            _ => panic!("invalid direction"),
        };
        let (dx, dy) = DIRECTIONS[ndir_idx];
        let (nx, ny) = (x as i64 + dx, y as i64 + dy);
        if blocks_moved <= max_block_moves
            && nx >= 0
            && nx < width as i64
            && ny >= 0
            && ny < height as i64
        {
            neighbours.push((nx as usize, ny as usize, ndir_idx, 1));
        }
        return neighbours;
    }
}

fn part1(grid: &Grid<i64>) -> i64 {
    let dists = find_min(grid, (0, 0), move |p, w, h| get_neighbours(p, w, h, 1, 3));
    dists
        .iter()
        .map(|d| d.iter().min().unwrap().clone())
        .min()
        .unwrap()
}

fn part2(grid: &Grid<i64>) -> i64 {
    let dists = find_min(grid, (0, 0), move |p, w, h| get_neighbours(p, w, h, 4, 10));
    dists
        .iter()
        // need to move atleast 4 before stopping
        .map(|dist| dist.iter().skip(4).min().unwrap().clone())
        .min()
        .unwrap()
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
