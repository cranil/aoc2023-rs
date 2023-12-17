use crate::grid;

pub mod priority_queue;

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
    return a / gcd(a, b) * b;
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
    let (g, u, _) = egcd(a, m);
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

pub fn binary_search<T: Ord>(arr: &[T], target: &T) -> Option<usize> {
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        if arr[m] == *target {
            return Some(m);
        } else if arr[m] < *target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    return None;
}

pub fn binary_search_by<T, F>(arr: &[T], target: &T, f: F) -> Option<usize>
where
    F: Fn(&T, &T) -> std::cmp::Ordering,
{
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        match f(&arr[m], target) {
            std::cmp::Ordering::Equal => return Some(m),
            std::cmp::Ordering::Less => l = m + 1,
            std::cmp::Ordering::Greater => r = m - 1,
        }
    }
    return None;
}

pub fn smallest_greater_than<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        if arr[m] == *target {
            return m + 1;
        } else if arr[m] < *target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    return l;
}

pub fn largest_less_than<T: Ord>(arr: &[T], target: &T) -> usize {
    let mut l = 0;
    let mut r = arr.len() - 1;
    while l <= r {
        let m = (l + r) / 2;
        if arr[m] == *target {
            return m - 1;
        } else if arr[m] < *target {
            l = m + 1;
        } else {
            r = m - 1;
        }
    }
    return r;
}

pub fn combinations(n: usize) -> grid::LowerTriangularGrid<usize> {
    let mut pascal = grid::LowerTriangularGrid::new(n + 1);
    for i in 0..=n {
        for j in 0..=i {
            if j == 0 || j == i {
                pascal.set(i, j, 1);
            } else {
                let y0 = pascal.at(i - 1, j - 1).unwrap();
                let y1 = pascal.at(i - 1, j).unwrap();
                pascal.set(i, j, y0 + y1);
            }
        }
    }
    return pascal;
}
