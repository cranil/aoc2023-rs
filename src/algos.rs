use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push(*self.at(x, y).unwrap());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<&str> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(*self.at(x, y).unwrap());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<String> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(self.at(x, y).unwrap().as_str());
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<i64> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!("{:^5}", *self.at(x, y).unwrap()));
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        for y in 0..self.height {
            for x in 0..self.width {
                s.push_str(&format!(
                    "{}",
                    if *self.at(x, y).unwrap() { "#" } else { "." }
                ));
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

#[allow(dead_code)]
impl<T: Default + Clone> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![<T as Default>::default(); width * height];
        return Self {
            data,
            width,
            height,
        };
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }
        return Some(&self.data[y * self.width + x]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[y * self.width + x] = value;
    }
}

pub struct LowerTriangularGrid<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T: Default + Clone> LowerTriangularGrid<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![<T as Default>::default(); size * (size + 1) / 2];
        return Self { data, size };
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x > y || y >= self.size {
            return None;
        }
        let index = y * (y + 1) / 2 + x;
        return Some(&self.data[index]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x > y || y >= self.size {
            return;
        }
        let index = y * (y + 1) / 2 + x;
        self.data[index] = value;
    }
}

pub struct UpperTriangularGrid<T> {
    pub data: Vec<T>,
    pub size: usize,
}

impl<T: Default + Clone> UpperTriangularGrid<T> {
    pub fn new(size: usize) -> Self {
        let data = vec![<T as Default>::default(); size * (size + 1) / 2];
        return Self { data, size };
    }

    pub fn at(&self, x: usize, y: usize) -> Option<&T> {
        if x < y || x >= self.size {
            return None;
        }
        let index = x * self.size - x * (x + 1) / 2 + y;
        return Some(&self.data[index]);
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if x < y || x >= self.size {
            return;
        }
        let index = x * self.size - x * (x + 1) / 2 + y;
        self.data[index] = value;
    }
}

pub fn gcd(a: i64, b: i64) -> i64 {
    let mut h = std::cmp::max(a, b);
    let mut l = std::cmp::min(a, b);
    if l == 0 {
        return h;
    }
    while l != 0 {
        let temp = l;
        l = h % l;
        h = temp;
    }
    return h;
}

pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    while r1 != 0 {
        let q = r0 / r1;
        let r2 = r0 - q * r1;
        let s2 = s0 - q * s1;
        let t2 = t0 - q * t1;
        r0 = r1;
        r1 = r2;
        s0 = s1;
        s1 = s2;
        t0 = t1;
        t1 = t2;
    }
    return (r0, s0, t0);
}

pub fn lcm(a: i64, b: i64) -> i64 {
    return a * b / gcd(a, b);
}

pub fn are_coprime(a: &[i64]) -> bool {
    for i in 0..a.len() {
        for j in i + 1..a.len() {
            if gcd(a[i], a[j]) != 1 {
                return false;
            }
        }
    }
    return true;
}

pub fn mod_inv(a: i64, m: i64) -> i64 {
    let (g, u, v) = egcd(a, m);
    assert!(g == 1);
    return (u % m + m) % m;
}

pub fn crt(a: &[i64], n: &[i64]) -> i64 {
    assert!(are_coprime(n));
    let prod = n.iter().product::<i64>();
    let mut sum = 0;
    for (&a_i, &n_i) in a.iter().zip(n.iter()) {
        let p = prod / n_i;
        sum += a_i * mod_inv(p, n_i) * p;
    }
    return sum % prod;
}
