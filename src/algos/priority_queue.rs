pub struct BinaryHeap<T, P: Ord> {
    pub(crate) data: Vec<(T, P)>,
}

impl<T, P: Ord> BinaryHeap<T, P> {
    pub fn new() -> Self {
        return Self { data: Vec::new() };
    }

    pub fn push(&mut self, val: T, priority: P) {
        self.data.push((val, priority));
        self.sift_up(self.data.len() - 1);
    }

    pub fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.data[parent].1 < self.data[idx].1 {
                break;
            }
            self.data.swap(parent, idx);
            idx = parent;
        }
    }

    pub fn pop(&mut self) -> Option<(T, P)> {
        if self.data.is_empty() {
            return None;
        }
        let val = self.data.swap_remove(0);
        self.sift_down(0);
        return Some(val);
    }

    pub fn sift_down(&mut self, mut idx: usize) {
        while idx < self.data.len() {
            let left = idx * 2 + 1;
            let right = idx * 2 + 2;
            let mut smallest = idx;
            if left < self.data.len() && self.data[left].1 < self.data[smallest].1 {
                smallest = left;
            }
            if right < self.data.len() && self.data[right].1 < self.data[smallest].1 {
                smallest = right;
            }
            if smallest == idx {
                break;
            }
            self.data.swap(smallest, idx);
            idx = smallest;
        }
    }

    pub fn peek(&self) -> Option<&(T, P)> {
        return self.data.get(0);
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    pub fn decrease_priority(&mut self, val: &T, priority: P)
    where
        T: PartialEq,
    {
        let idx = self
            .data
            .iter()
            .position(|(v, _)| *v == *val)
            .expect("Value not found");
        self.data[idx].1 = priority;
        self.sift_up(idx);
    }
}
