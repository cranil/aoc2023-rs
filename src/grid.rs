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

#[derive(Debug, Clone)]
pub struct DynamicGrid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Default + Clone> DynamicGrid<T> {
    pub fn new() -> Self {
        let data = Vec::new();
        return Self {
            data,
            width: 0,
            height: 0,
        };
    }

    pub fn with_capacity(width: usize, height: usize) -> Self {
        let data = Vec::with_capacity(width * height);
        return Self {
            data,
            width: 0,
            height: 0,
        };
    }

    pub fn fill(&mut self, value: T) {
        self.data.fill(value);
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

    pub fn resize(&mut self, width: usize, height: usize) {
        let mut new_data = vec![<T as Default>::default(); width * height];
        for y in 0..std::cmp::min(self.height, height) {
            for x in 0..std::cmp::min(self.width, width) {
                new_data[y * width + x] = self.data[y * self.width + x].clone();
            }
        }
        self.data = new_data;
        self.width = width;
        self.height = height;
    }

    pub fn insert_row(&mut self, index: usize) {
        self.data.reserve(self.width);
        for _ in 0..self.width {
            self.data
                .insert(index * self.width, <T as Default>::default());
        }
        self.height += 1;
    }

    pub fn insert_column(&mut self, index: usize) {
        self.data.reserve(self.height);
        for y in 0..self.height {
            self.data
                .insert(index + y * (self.width + 1), <T as Default>::default());
        }
        self.width += 1;
    }

    pub fn insert_column_with(&mut self, index: usize, value: T) {
        self.data.reserve(self.height);
        for y in 0..self.height {
            self.data
                .insert(index + y * (self.width + 1), value.clone());
        }
        self.width += 1;
    }
}

#[cfg(test)]
mod test_grid {
    #[test]
    fn at() {
        let mut grid = super::Grid::<i32>::new(3, 3);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);
        grid.set(2, 0, 3);
        grid.set(0, 1, 4);
        grid.set(1, 1, 5);
        grid.set(2, 1, 6);
        grid.set(0, 2, 7);
        grid.set(1, 2, 8);
        grid.set(2, 2, 9);
        assert_eq!(grid.at(0, 0), Some(&1));
        assert_eq!(grid.at(1, 0), Some(&2));
        assert_eq!(grid.at(2, 0), Some(&3));
        assert_eq!(grid.at(0, 1), Some(&4));
        assert_eq!(grid.at(1, 1), Some(&5));
        assert_eq!(grid.at(2, 1), Some(&6));
        assert_eq!(grid.at(0, 2), Some(&7));
        assert_eq!(grid.at(1, 2), Some(&8));
        assert_eq!(grid.at(2, 2), Some(&9));
        assert_eq!(grid.at(3, 2), None);
        assert_eq!(grid.at(2, 3), None);
    }
}

#[cfg(test)]
mod test_lower_triangular_grid {
    #[test]
    fn at() {
        let mut grid = super::LowerTriangularGrid::<i32>::new(3);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);
        grid.set(1, 1, 3);
        grid.set(2, 0, 4);
        grid.set(2, 1, 5);
        grid.set(2, 2, 6);
        assert_eq!(grid.at(0, 0), Some(&1));
        assert_eq!(grid.at(1, 0), Some(&2));
        assert_eq!(grid.at(1, 1), Some(&3));
        assert_eq!(grid.at(2, 0), Some(&4));
        assert_eq!(grid.at(2, 1), Some(&5));
        assert_eq!(grid.at(2, 2), Some(&6));
        assert_eq!(grid.at(3, 2), None);
        assert_eq!(grid.at(2, 3), None);
    }
}

#[cfg(test)]
mod test_upper_triangular_grid {
    #[test]
    fn at() {
        let mut grid = super::UpperTriangularGrid::<i32>::new(3);
        grid.set(0, 0, 1);
        grid.set(0, 1, 2);
        grid.set(1, 1, 3);
        grid.set(0, 2, 4);
        grid.set(1, 2, 5);
        grid.set(2, 2, 6);
        assert_eq!(grid.at(0, 0), Some(&1));
        assert_eq!(grid.at(0, 1), Some(&2));
        assert_eq!(grid.at(1, 1), Some(&3));
        assert_eq!(grid.at(0, 2), Some(&4));
        assert_eq!(grid.at(1, 2), Some(&5));
        assert_eq!(grid.at(2, 2), Some(&6));
        assert_eq!(grid.at(3, 2), None);
        assert_eq!(grid.at(2, 3), None);
    }
}

#[cfg(test)]
mod test_dynamic_grid {
    #[test]
    fn insert_row() {
        let mut grid = super::DynamicGrid::<i32>::new();
        grid.resize(3, 3);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);
        grid.set(2, 0, 3);
        grid.set(0, 1, 4);
        grid.set(1, 1, 5);
        grid.set(2, 1, 6);
        grid.set(0, 2, 7);
        grid.set(1, 2, 8);
        grid.set(2, 2, 9);
        grid.insert_row(1);
        assert_eq!(grid.width, 3);
        assert_eq!(grid.height, 4);
        assert_eq!(grid.at(0, 0), Some(&1));
        assert_eq!(grid.at(1, 0), Some(&2));
        assert_eq!(grid.at(2, 0), Some(&3));
        assert_eq!(grid.at(0, 1), Some(&0));
        assert_eq!(grid.at(1, 1), Some(&0));
        assert_eq!(grid.at(2, 1), Some(&0));
        assert_eq!(grid.at(0, 2), Some(&4));
        assert_eq!(grid.at(1, 2), Some(&5));
        assert_eq!(grid.at(2, 2), Some(&6));
        assert_eq!(grid.at(0, 3), Some(&7));
        assert_eq!(grid.at(1, 3), Some(&8));
        assert_eq!(grid.at(2, 3), Some(&9));
    }

    #[test]
    fn insert_column() {
        let mut grid = super::DynamicGrid::<i32>::new();
        grid.resize(3, 3);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);
        grid.set(2, 0, 3);
        grid.set(0, 1, 4);
        grid.set(1, 1, 5);
        grid.set(2, 1, 6);
        grid.set(0, 2, 7);
        grid.set(1, 2, 8);
        grid.set(2, 2, 9);
        grid.insert_column(1);
        assert_eq!(grid.width, 4);
        assert_eq!(grid.height, 3);
        assert_eq!(grid.at(0, 0), Some(&1));
        assert_eq!(grid.at(1, 0), Some(&0));
        assert_eq!(grid.at(2, 0), Some(&2));
        assert_eq!(grid.at(3, 0), Some(&3));
        assert_eq!(grid.at(0, 1), Some(&4));
        assert_eq!(grid.at(1, 1), Some(&0));
        assert_eq!(grid.at(2, 1), Some(&5));
        assert_eq!(grid.at(3, 1), Some(&6));
        assert_eq!(grid.at(0, 2), Some(&7));
        assert_eq!(grid.at(1, 2), Some(&0));
        assert_eq!(grid.at(2, 2), Some(&8));
        assert_eq!(grid.at(3, 2), Some(&9));
    }
}
