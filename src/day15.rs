use crate::utils::{main, read_all, test};

fn get_contents(filename: &str) -> Vec<String> {
    let mut contents = read_all(filename);
    contents = contents.replace("\n", "");
    return contents.split(",").map(String::from).collect();
}

fn elf_hash(string: &String) -> usize {
    let mut hash = 0;
    for c in string.chars() {
        let cval = c as usize;
        hash += cval;
        hash *= 17;
        hash %= 256;
    }
    return hash;
}

fn part1(strings: &Vec<String>) -> usize {
    return strings.iter().map(elf_hash).sum::<usize>();
}

fn part2(strings: &Vec<String>) -> usize {
    let mut boxes = vec![Vec::<(String, usize)>::new(); 256];
    for string in strings.iter() {
        if string.contains("=") {
            let mut iter = string.split("=");
            let key = iter.next().unwrap().to_string();
            let value = iter.next().unwrap().parse::<usize>().unwrap();
            let hash = elf_hash(&key);
            let bucket = &mut boxes[hash as usize];
            if let Some((_, v)) = bucket.iter_mut().find(|(k, _)| *k == key) {
                *v = value;
            } else {
                bucket.push((key, value));
            }
        } else if string.contains("-") {
            let mut iter = string.split("-");
            let key = iter.next().unwrap().to_string();
            let bucket = &mut boxes[elf_hash(&key) as usize];
            if let Some(pos) = bucket.iter().position(|(k, _)| *k == key) {
                bucket.remove(pos);
            }
        }
    }
    return boxes
        .iter()
        .enumerate()
        .map(|(bx, v)| {
            v.iter()
                .enumerate()
                .map(|(slt, (_, val))| val * (slt + 1) * (bx + 1))
                .sum::<usize>()
        })
        .sum::<usize>();
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
