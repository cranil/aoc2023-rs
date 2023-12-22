use std::collections::{HashSet, VecDeque};

use crate::{
    grid::Grid,
    utils::{main, read_lines, test},
};

#[derive(Clone, Debug)]
struct Brick {
    from: (isize, isize, isize),
    to: (isize, isize, isize),
}

type InputData = Vec<Brick>;

fn get_contents(filename: &str) -> InputData {
    read_lines(filename)
        .iter()
        .map(|line| {
            let mut parts = line.split('~');
            let from = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let to = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            Brick {
                from: (from[0], from[1], from[2]),
                to: (to[0], to[1], to[2]),
            }
        })
        .collect()
}

fn get_blocks(brick: &Brick) -> Vec<(isize, isize, isize)> {
    let mut blocks = Vec::new();
    for z in brick.from.2..=brick.to.2 {
        for y in brick.from.1..=brick.to.1 {
            for x in brick.from.0..=brick.to.0 {
                blocks.push((x, y, z));
            }
        }
    }
    blocks
}

fn get_z(brick_num: usize, brick: &Brick, grid: &Vec<Grid<usize>>) -> isize {
    let mut z = 0;
    let blocks = get_blocks(brick);
    for zp in (1..brick.from.2 as usize).rev() {
        let mut all_free = true;
        for (x, y, _) in blocks.iter() {
            if let Some(c) = grid[zp].at(*x as usize, *y as usize) {
                if *c != 0 || *c == brick_num + 1 {
                    all_free = false;
                    break;
                }
            }
        }
        if !all_free {
            break;
        }
        z = zp as isize;
    }
    if z == 0 {
        z = blocks.iter().map(|(_, _, z)| *z).min().unwrap();
    }
    z
}

fn find_supports(bricks: &InputData) -> (Vec<HashSet<usize>>, Vec<HashSet<usize>>) {
    let mut bricks = bricks.clone();
    let mut x_max = 0;
    let mut y_max = 0;
    let mut z_max = 0;

    let mut x_min = isize::MAX;
    let mut y_min = isize::MAX;
    let mut z_min = isize::MAX;

    for brick in bricks.iter() {
        x_max = x_max.max(brick.from.0).max(brick.to.0);
        y_max = y_max.max(brick.from.1).max(brick.to.1);
        z_max = z_max.max(brick.from.2).max(brick.to.2);

        x_min = x_min.min(brick.from.0).min(brick.to.0);
        y_min = y_min.min(brick.from.1).min(brick.to.1);
        z_min = z_min.min(brick.from.2).min(brick.to.2);
    }

    let mut grid = vec![Grid::new(1 + x_max as usize, 1 + y_max as usize); 1 + z_max as usize];

    let mut sorted_map = (0..bricks.len()).collect::<Vec<_>>();
    sorted_map.sort_by_key(|i| bricks[*i].from.2);

    for i in sorted_map.iter() {
        let brick = &mut bricks[*i];
        let zp = get_z(*i, brick, &grid);
        let blocks = get_blocks(brick);

        let zdiff = brick.from.2 - zp;
        for (x, y, z) in blocks.iter() {
            grid[(z - zdiff) as usize].set(*x as usize, *y as usize, *i + 1);
        }

        brick.to.2 -= zdiff;
        brick.from.2 -= zdiff;
    }

    let mut supports = vec![HashSet::new(); bricks.len()];
    let mut supported_by = vec![HashSet::new(); bricks.len()];
    for z in 0..z_max as usize {
        let curr = &grid[z];
        let above = &grid[z + 1];
        for y in 0..=y_max as usize {
            for x in 0..=x_max as usize {
                let curr_pt = curr.at(x, y).unwrap();
                let above_pt = above.at(x, y).unwrap();
                if *curr_pt == 0 {
                    continue;
                }
                if *above_pt == 0 || *above_pt == *curr_pt {
                    continue;
                }
                supports[*curr_pt as usize - 1].insert(*above_pt);
                supported_by[*above_pt as usize - 1].insert(*curr_pt);
            }
        }
    }
    (supports, supported_by)
}

fn part1(bricks: &InputData) -> i64 {
    let (supports, supported_by) = find_supports(bricks);
    let mut can_disintegrate = 0;
    for (i, _) in bricks.iter().enumerate() {
        let mut all_supported = true;
        for brick_above in supports[i].iter() {
            if supported_by[*brick_above as usize - 1].len() == 1 {
                all_supported = false;
                break;
            }
        }
        if all_supported {
            can_disintegrate += 1;
        }
    }
    can_disintegrate as i64
}

fn part2(bricks: &InputData) -> i64 {
    let (supports, supported_by) = find_supports(bricks);

    let mut affects = vec![0; bricks.len()];
    for i in 0..bricks.len() {
        let mut affected = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(i);
        affected.insert(i);
        while let Some(brick_idx) = queue.pop_front() {
            for brick_above in supports[brick_idx].iter() {
                let mut topples = true;
                for supp in supported_by[*brick_above as usize - 1].iter() {
                    if !affected.contains(&(*supp - 1)) {
                        topples = false;
                        break;
                    }
                }
                if topples {
                    queue.push_back(*brick_above - 1);
                    affected.insert(*brick_above - 1);
                }
            }
        }
        affects[i] = affected.len() - 1;
    }
    affects.iter().sum::<usize>() as i64
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
