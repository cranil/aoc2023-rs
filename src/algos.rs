pub struct Grid<T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
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
