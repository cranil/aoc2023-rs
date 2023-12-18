use crate::utils::{main, read_lines, test};

fn get_contents(filename: &str) -> Vec<(char, i64, String)> {
    let lines = read_lines(filename);
    lines
        .iter()
        .map(|line| {
            let mut toks = line.split_whitespace();
            let c = toks.next().unwrap().chars().next().unwrap();
            let dist = toks.next().unwrap().parse::<i64>().unwrap();
            let colour = toks.next().unwrap().to_string();
            let colour = colour[1..colour.len() - 1].to_string();
            (c, dist, colour)
        })
        .collect()
}

fn part1(plan: &Vec<(char, i64, String)>) -> i64 {
    let mut area = 0;
    let mut xcurr = 0;
    let mut ycurr = 0;
    let mut x_min = 0;
    let mut x_max = 0;
    let mut y_min = 0;
    let mut y_max = 0;
    let mut num_points = 0;
    for (c, dist, _) in plan.iter() {
        let instr = match c {
            'R' => 0,
            'D' => 1,
            'L' => 2,
            'U' => 3,
            _ => panic!("invalid direction"),
        };
        let dist = *dist;
        num_points += dist;
        xcurr += match instr {
            0 => dist,
            1 => 0,
            2 => -dist,
            3 => 0,
            _ => panic!("invalid direction"),
        };
        ycurr += match instr {
            0 => 0,
            1 => dist,
            2 => 0,
            3 => -dist,
            _ => panic!("invalid direction"),
        };
        x_min = x_min.min(xcurr);
        x_max = x_max.max(xcurr);
        y_min = y_min.min(ycurr);
        y_max = y_max.max(ycurr);

        let (dx, dy) = match instr {
            0 => (dist, 0),
            1 => (0, dist),
            2 => (-dist, 0),
            3 => (0, -dist),
            _ => panic!("invalid direction"),
        };
        // green's theorem (div by 2 in the end to avoid precision issues)
        area += -dx * ycurr + dy * xcurr + dist * 1;
    }
    area / 2 + 1
}

fn parse_hex(s: &str) -> i64 {
    let mut n = 0;
    for c in s.chars() {
        n *= 16;
        n += match c {
            '0'..='9' => c as i64 - '0' as i64,
            'a'..='f' => c as i64 - 'a' as i64 + 10,
            _ => panic!("invalid hex"),
        };
    }
    n
}

fn part2(plan: &Vec<(char, i64, String)>) -> i64 {
    let mut area = 0;

    let mut xcurr = 0;
    let mut ycurr = 0;

    let mut x_min = 0;
    let mut x_max = 0;

    let mut y_min = 0;
    let mut y_max = 0;
    let mut num_points = 0;

    for (_, _, code) in plan.iter() {
        let instr = code
            .chars()
            .last()
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap();
        let dist = parse_hex(&code[1..code.len() - 1]);
        num_points += dist;
        xcurr += match instr {
            0 => dist,
            1 => 0,
            2 => -dist,
            3 => 0,
            _ => panic!("invalid direction"),
        };
        ycurr += match instr {
            0 => 0,
            1 => dist,
            2 => 0,
            3 => -dist,
            _ => panic!("invalid direction"),
        };
        x_min = x_min.min(xcurr);
        x_max = x_max.max(xcurr);
        y_min = y_min.min(ycurr);
        y_max = y_max.max(ycurr);

        let (dx, dy) = match instr {
            0 => (dist, 0),
            1 => (0, dist),
            2 => (-dist, 0),
            3 => (0, -dist),
            _ => panic!("invalid direction"),
        };
        area += -dx * ycurr + dy * xcurr + dist * 1;
    }
    area / 2 + 1
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
