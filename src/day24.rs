use crate::utils::{main, read_lines, test};

type InputData = Vec<HailStone>;

#[derive(Debug, Clone, Copy)]
struct BoundingBox {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl BoundingBox {
    fn new(min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    fn contains(&self, point: &Point) -> bool {
        let Point { x, y, .. } = *point;
        self.min_x <= x && x <= self.max_x && self.min_y <= y && y <= self.max_y
    }
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
struct HailStone {
    position: Point,
    velocity: Point,
}

fn solve_linear(
    (a1, b1, c1): (f64, f64, f64),
    (a2, b2, c2): (f64, f64, f64),
) -> Option<(f64, f64)> {
    let det = a1 * b2 - a2 * b1;
    if det == 0.0 {
        None
    } else {
        let x = (b2 * c1 - b1 * c2) / det;
        let y = (a1 * c2 - a2 * c1) / det;
        Some((x, y))
    }
}

impl HailStone {
    fn new(position: Point, velocity: Point) -> Self {
        Self { position, velocity }
    }

    fn intersection_xy(&self, other: &Self) -> Option<Point> {
        let (x0, y0) = (self.position.x, self.position.y);
        let (x1, y1) = (other.position.x, other.position.y);
        let (vx0, vy0) = (self.velocity.x, self.velocity.y);
        let (vx1, vy1) = (other.velocity.x, other.velocity.y);

        let c0 = x1 - x0;
        let c1 = y1 - y0;
        let (a0, b0) = (vx0, -vx1);
        let (a1, b1) = (vy0, -vy1);

        if let Some((s, t)) = solve_linear(
            (a0 as f64, b0 as f64, c0 as f64),
            (a1 as f64, b1 as f64, c1 as f64),
        ) {
            if s >= 0.0 && t >= 0.0 {
                let xs = x0 as f64 + vx0 as f64 * s;
                let ys = y0 as f64 + vy0 as f64 * s;
                let xt = x1 as f64 + vx1 as f64 * t;
                let yt = y1 as f64 + vy1 as f64 * t;

                let x = (xs + xt) / 2.0;
                let y = (ys + yt) / 2.0;

                Some(Point::new(x as i64, y as i64, 0))
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let s = s.trim();
        let mut iter = s.split(',');
        let x = iter.next().unwrap().trim().parse::<i64>().unwrap();
        let y = iter.next().unwrap().trim().parse::<i64>().unwrap();
        let z = iter.next().unwrap().trim().parse::<i64>().unwrap();
        Self::new(x, y, z)
    }
}

fn get_contents(filename: &str) -> InputData {
    let lines = read_lines(filename);
    lines
        .iter()
        .map(|s| {
            let mut iter = s.split("@");
            let point_str = iter.next().unwrap().trim();
            let vel_str = iter.next().unwrap().trim();
            HailStone::new(point_str.into(), vel_str.into())
        })
        .collect()
}

fn part1(hail_stones: &InputData) -> i64 {
    let bounding_box = BoundingBox::new(
        200000000000000,
        400000000000000,
        200000000000000,
        400000000000000,
    );
    let mut count = 0;
    for i in 0..hail_stones.len() {
        for j in i + 1..hail_stones.len() {
            let hail_stone1 = &hail_stones[i];
            let hail_stone2 = &hail_stones[j];
            if let Some(intersection) = hail_stone1.intersection_xy(hail_stone2) {
                if bounding_box.contains(&intersection) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn gauss_elimination(matrix: &mut [[f64; 7]; 6]) {
    let n = matrix.len();
    for i in 0..n {
        let mut max_row = i;
        for j in i + 1..n {
            if matrix[j][i].abs() > matrix[max_row][i].abs() {
                max_row = j;
            }
        }

        for k in i..n + 1 {
            let tmp = matrix[max_row][k];
            matrix[max_row][k] = matrix[i][k];
            matrix[i][k] = tmp;
        }

        for j in i + 1..n {
            let c = matrix[j][i] / matrix[i][i];
            for k in i..n + 1 {
                if i == k {
                    matrix[j][k] = 0.0;
                } else {
                    matrix[j][k] -= c * matrix[i][k];
                }
            }
        }
    }

    for i in (0..n).rev() {
        matrix[i][n] /= matrix[i][i];
        matrix[i][i] = 1.0;
        for j in 0..i {
            matrix[j][n] -= matrix[j][i] * matrix[i][n];
            matrix[j][i] = 0.0;
        }
    }
}

fn cross(a: &Point) -> [[i64; 3]; 3] {
    [[0, -a.z, a.y], [a.z, 0, -a.x], [-a.y, a.x, 0]]
}

fn matvecmul(matrix: [[i64; 3]; 3], b: [i64; 3]) -> [i64; 3] {
    let mut c = [0; 3];
    for i in 0..3 {
        for j in 0..3 {
            c[i] += matrix[i][j] * b[j];
        }
    }
    c
}

fn part2(hail_stones: &InputData) -> i64 {
    let mut hail_stones = hail_stones.clone();
    let cmp = |p: &HailStone, q: &HailStone| {
        let p_size = p.position.x.abs() + p.position.y.abs() + p.position.z.abs();
        let q_size = q.position.x.abs() + q.position.y.abs() + q.position.z.abs();
        let p = p.position;
        let q = q.position;
        p_size
            .cmp(&q_size)
            .then(p.x.cmp(&q.x).then(p.y.cmp(&q.y)).then(p.z.cmp(&q.z)))
    };
    hail_stones.sort_by(cmp);

    let p0 = hail_stones[0].position;
    let p1 = hail_stones[1].position;
    let p2 = hail_stones[2].position;

    let v0 = hail_stones[0].velocity;
    let v1 = hail_stones[1].velocity;
    let v2 = hail_stones[2].velocity;

    let dp0 = Point::new(p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
    let dp1 = Point::new(p2.x - p1.x, p2.y - p1.y, p2.z - p1.z);

    let dv0 = Point::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
    let dv1 = Point::new(v2.x - v1.x, v2.y - v1.y, v2.z - v1.z);

    let dp0x = cross(&dp0);
    let dp1x = cross(&dp1);

    let dv0x = cross(&dv0);
    let dv1x = cross(&dv1);

    // M = [[-dv0x, dp0x], [-dv1x, dp1x]]
    let mut coeffs = [[0.0; 7]; 6];
    for i in 0..3 {
        for j in 0..3 {
            coeffs[i][j] = -dv0x[i][j] as f64;
            coeffs[i][j + 3] = dp0x[i][j] as f64;
            coeffs[i + 3][j] = -dv1x[i][j] as f64;
            coeffs[i + 3][j + 3] = dp1x[i][j] as f64;
        }
    }

    // B = [p1xv1 - p0xv0, p2xv2 - p0xv0]
    let p0x = cross(&p0);
    let p1x = cross(&p1);
    let p2x = cross(&p2);

    let p0xv0 = matvecmul(p0x, [v0.x, v0.y, v0.z]);
    let p1xv1 = matvecmul(p1x, [v1.x, v1.y, v1.z]);
    let p2xv2 = matvecmul(p2x, [v2.x, v2.y, v2.z]);

    for i in 0..3 {
        coeffs[i][6] = (p1xv1[i] - p0xv0[i]) as f64;
        coeffs[i + 3][6] = (p2xv2[i] - p1xv1[i]) as f64;
    }
    gauss_elimination(&mut coeffs);

    let rock_pos = Point::new(
        coeffs[0][6] as i64,
        coeffs[1][6] as i64,
        coeffs[2][6] as i64,
    );
    let rock_vel = Point::new(
        coeffs[3][6] as i64,
        coeffs[4][6] as i64,
        coeffs[5][6] as i64,
    );

    // fix rounding errors
    for dx in [-1, 0, 1] {
        for dy in [-1, 0, 1] {
            for dz in [-1, 0, 1] {
                for dvx in [-1, 0, 1] {
                    for dvy in [-1, 0, 1] {
                        for dvz in [-1, 0, 1] {
                            let hpos = hail_stones[0].position;
                            let hvel = hail_stones[0].velocity;

                            let pos_diff = Point::new(
                                hpos.x - rock_pos.x - dx,
                                hpos.y - rock_pos.y - dy,
                                hpos.z - rock_pos.z - dz,
                            );
                            let vel_diff = Point::new(
                                hvel.x - rock_vel.x - dvx,
                                hvel.y - rock_vel.y - dvy,
                                hvel.z - rock_vel.z - dvz,
                            );

                            let pos_diffx = cross(&pos_diff);
                            let prod = matvecmul(pos_diffx, [vel_diff.x, vel_diff.y, vel_diff.z]);
                            if prod[0] == 0 && prod[1] == 0 && prod[2] == 0 {
                                return rock_pos.x + rock_pos.y + rock_pos.z + dx + dy + dz;
                            }
                        }
                    }
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
