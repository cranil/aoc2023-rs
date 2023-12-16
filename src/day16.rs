use std::{cmp::max, collections::VecDeque};

use crate::{
    grid::Grid,
    utils::{main, test},
};

fn get_contents(filename: &str) -> Grid<char> {
    let lines = crate::utils::read_lines(filename);
    let mut grid = Grid::new(lines.len(), lines[0].len());
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid.set(x, y, c);
        }
    }
    return grid;
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Ray {
    point: (i64, i64),
    dir: (i64, i64),
}

struct Visited {
    grid: Grid<u8>,
}

impl Visited {
    fn new(width: usize, height: usize) -> Visited {
        return Visited {
            grid: Grid::new(width, height),
        };
    }

    fn contains(&self, ray: &Ray) -> bool {
        let (x, y) = ray.point;
        let (dx, dy) = ray.dir;
        let val = self.grid.at(x as usize, y as usize).unwrap();
        let cell_visited = val & 0b0000_0001 != 0;
        let dir_visited = match (dx, dy) {
            (0, 1) => val & 0b0000_0010 != 0,
            (0, -1) => val & 0b0000_0100 != 0,
            (1, 0) => val & 0b0000_1000 != 0,
            (-1, 0) => val & 0b0001_0000 != 0,
            _ => panic!("Unknown direction {:?}", ray),
        };
        return cell_visited && dir_visited;
    }

    fn visit(&mut self, ray: &Ray) {
        let (x, y) = ray.point;
        let (dx, dy) = ray.dir;
        if x < 0 || x >= self.grid.width as i64 || y < 0 || y >= self.grid.height as i64 {
            return;
        }
        let val = 0b0000_0001
            | match (dx, dy) {
                (0, 1) => 0b0000_0010,
                (0, -1) => 0b0000_0100,
                (1, 0) => 0b0000_1000,
                (-1, 0) => 0b0001_0000,
                _ => panic!("Unknown direction {:?}", ray),
            };
        self.grid.set(x as usize, y as usize, val);
    }
}

fn energize_bfs(ray: &Ray, grid: &Grid<char>) -> Grid<u8> {
    let mut visited_ray = Visited::new(grid.width, grid.height);

    let mut queue = VecDeque::new();
    queue.push_back(ray.clone());
    while !queue.is_empty() {
        let ray = queue.pop_front().unwrap();
        let (x, y) = ray.point;
        let (dx, dy) = ray.dir;
        let (nx, ny) = (x + dx, y + dy);
        if nx < 0 || nx >= grid.width as i64 || ny < 0 || ny >= grid.height as i64 {
            continue;
        }
        let c = grid.at(nx as usize, ny as usize).unwrap();
        match c {
            '.' => {
                let new_ray = Ray {
                    point: (nx, ny),
                    dir: (dx, dy),
                };
                if !visited_ray.contains(&new_ray) {
                    queue.push_back(new_ray.clone());
                    visited_ray.visit(&new_ray);
                }
            }
            '/' => {
                let new_ray = Ray {
                    point: (nx, ny),
                    dir: (-dy, -dx),
                };
                if !visited_ray.contains(&new_ray) {
                    queue.push_back(new_ray.clone());
                    visited_ray.visit(&new_ray);
                }
            }
            '\\' => {
                let new_ray = Ray {
                    point: (nx, ny),
                    dir: (dy, dx),
                };
                if !visited_ray.contains(&new_ray) {
                    queue.push_back(new_ray.clone());
                    visited_ray.visit(&new_ray);
                }
            }
            '|' => {
                if dx == 0 {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (dx, dy),
                    };
                    if !visited_ray.contains(&new_ray) {
                        queue.push_back(new_ray.clone());
                        visited_ray.visit(&new_ray);
                    }
                } else {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (0, 1),
                    };
                    if !visited_ray.contains(&new_ray) {
                        queue.push_back(new_ray.clone());
                        visited_ray.visit(&new_ray);
                    }
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (0, -1),
                    };
                    if !visited_ray.contains(&new_ray) {
                        queue.push_back(new_ray.clone());
                        visited_ray.visit(&new_ray);
                    }
                }
            }
            '-' => {
                if dy == 0 {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (dx, dy),
                    };
                    if !visited_ray.contains(&new_ray) {
                        queue.push_back(new_ray.clone());
                        visited_ray.visit(&new_ray);
                    }
                } else {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (1, 0),
                    };
                    if !visited_ray.contains(&new_ray) {
                        queue.push_back(new_ray.clone());
                        visited_ray.visit(&new_ray);
                    }

                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (-1, 0),
                    };
                    if !visited_ray.contains(&new_ray) {
                        queue.push_back(new_ray.clone());
                        visited_ray.visit(&new_ray);
                    }
                }
            }
            _ => panic!("Unknown character {}", c),
        }
    }
    return visited_ray.grid;
}

fn count_energized(grid: &Grid<u8>) -> i64 {
    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.at(x, y).unwrap() & 0b0000_0001 != 0 {
                count += 1;
            }
        }
    }
    return count;
}

fn part1(grid: &Grid<char>) -> i64 {
    let ray = Ray {
        point: (-1, 0),
        dir: (1, 0),
    };
    let light_grid = energize_bfs(&ray, grid);
    return count_energized(&light_grid);
}

fn part2(grid: &Grid<char>) -> i64 {
    let right_max = (0..grid.height)
        .map(|y| Ray {
            point: (-1, y as i64),
            dir: (1, 0),
        })
        .map(|ray| {
            let rays = vec![ray];
            energize_bfs(&rays[0], grid)
        })
        .map(|grid| count_energized(&grid))
        .max()
        .unwrap();
    let left_max = (0..grid.height)
        .map(|y| Ray {
            point: (grid.width as i64, y as i64),
            dir: (-1, 0),
        })
        .map(|ray| {
            let rays = vec![ray];
            energize_bfs(&rays[0], grid)
        })
        .map(|grid| count_energized(&grid))
        .max()
        .unwrap();
    let top_max = (0..grid.width)
        .map(|x| Ray {
            point: (x as i64, -1),
            dir: (0, 1),
        })
        .map(|ray| {
            let rays = vec![ray];
            energize_bfs(&rays[0], grid)
        })
        .map(|grid| count_energized(&grid))
        .max()
        .unwrap();
    let bottom_max = (0..grid.width)
        .map(|x| Ray {
            point: (x as i64, grid.height as i64),
            dir: (0, -1),
        })
        .map(|ray| {
            let rays = vec![ray];
            energize_bfs(&rays[0], grid)
        })
        .map(|grid| count_energized(&grid))
        .max()
        .unwrap();
    return max(max(right_max, left_max), max(top_max, bottom_max));
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
