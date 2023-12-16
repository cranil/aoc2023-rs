use std::{
    cmp::max,
    collections::{HashMap, HashSet},
};

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

fn step(rays: Vec<Ray>, grid: &Grid<char>, ray_set: &HashSet<Ray>) -> Vec<Ray> {
    let mut new_rays = Vec::new();
    for ray in rays.iter() {
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
                if !ray_set.contains(&new_ray) {
                    new_rays.push(new_ray);
                }
            }
            '/' => {
                let new_ray = Ray {
                    point: (nx, ny),
                    dir: (-dy, -dx),
                };
                if !ray_set.contains(&new_ray) {
                    new_rays.push(new_ray);
                }
            }
            '\\' => {
                let new_ray = Ray {
                    point: (nx, ny),
                    dir: (dy, dx),
                };
                if !ray_set.contains(&new_ray) {
                    new_rays.push(new_ray);
                }
            }
            '|' => {
                if dx == 0 {
                    new_rays.push(Ray {
                        point: (nx, ny),
                        dir: (dx, dy),
                    });
                } else {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (0, 1),
                    };
                    if !ray_set.contains(&new_ray) {
                        new_rays.push(new_ray);
                    }
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (0, -1),
                    };
                    if !ray_set.contains(&new_ray) {
                        new_rays.push(new_ray);
                    }
                }
            }
            '-' => {
                if dy == 0 {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (dx, dy),
                    };
                    if !ray_set.contains(&new_ray) {
                        new_rays.push(new_ray);
                    }
                } else {
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (1, 0),
                    };
                    if !ray_set.contains(&new_ray) {
                        new_rays.push(new_ray);
                    }
                    let new_ray = Ray {
                        point: (nx, ny),
                        dir: (-1, 0),
                    };
                    if !ray_set.contains(&new_ray) {
                        new_rays.push(new_ray);
                    }
                }
            }
            _ => panic!("Unknown character {}", c),
        }
    }
    return new_rays;
}

fn part1(grid: &Grid<char>) -> i64 {
    let mut dp = HashMap::new();
    let rays = vec![Ray {
        point: (-1, 0),
        dir: (1, 0),
    }];
    let light_grid = energize(&rays, grid, &mut dp);
    return count_energized(&light_grid);
}

fn union(grid1: &Grid<bool>, grid2: &Grid<bool>) -> Grid<bool> {
    let mut grid = Grid::new(grid1.width, grid1.height);
    for y in 0..grid.height {
        for x in 0..grid.width {
            grid.set(x, y, *grid1.at(x, y).unwrap() || *grid2.at(x, y).unwrap());
        }
    }
    return grid;
}

fn energize_ray(ray: &Ray, grid: &Grid<char>) -> Grid<bool> {
    let mut energized = Grid::new(grid.width, grid.height);
    let mut rays = vec![ray.clone()];
    if ray.point.0 >= 0
        && ray.point.0 < grid.width as i64
        && ray.point.1 >= 0
        && ray.point.1 < grid.height as i64
    {
        energized.set(ray.point.0 as usize, ray.point.1 as usize, true);
    }

    let mut light_trail = HashSet::new();

    while !rays.is_empty() {
        rays = step(rays, grid, &light_trail);
        for ray in rays.iter() {
            light_trail.insert(ray.clone());
            if ray.point.0 < 0
                || ray.point.0 >= grid.width as i64
                || ray.point.1 < 0
                || ray.point.1 >= grid.height as i64
            {
                continue;
            }
            energized.set(ray.point.0 as usize, ray.point.1 as usize, true);
        }
    }
    return energized;
}

fn energize(rays: &Vec<Ray>, grid: &Grid<char>, dp: &mut HashMap<Ray, Grid<bool>>) -> Grid<bool> {
    let mut energized = Grid::new(grid.width, grid.height);
    for ray in rays.iter() {
        if let Some(energized_ray) = dp.get(ray) {
            energized = union(&energized, energized_ray);
        } else {
            let energized_ray = energize_ray(ray, grid);
            energized = union(&energized, &energized_ray);
            dp.insert(ray.clone(), energized_ray);
        }
    }
    return energized;
}

fn count_energized(grid: &Grid<bool>) -> i64 {
    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            if *grid.at(x, y).unwrap() {
                count += 1;
            }
        }
    }
    return count;
}

fn part2(grid: &Grid<char>) -> i64 {
    let mut dp = HashMap::new();

    let right_max = (0..grid.height)
        .map(|y| Ray {
            point: (-1, y as i64),
            dir: (1, 0),
        })
        .map(|ray| {
            let rays = vec![ray];
            energize(&rays, grid, &mut dp)
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
            energize(&rays, grid, &mut dp)
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
            energize(&rays, grid, &mut dp)
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
            energize(&rays, grid, &mut dp)
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
