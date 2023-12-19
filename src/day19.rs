use std::{collections::HashMap, fmt::Display};

use crate::utils::{main, read_lines, test};

#[derive(Debug)]
struct Part {
    a: i64,
    m: i64,
    s: i64,
    x: i64,
}

impl Part {
    fn new(a: i64, m: i64, s: i64, x: i64) -> Self {
        Self { a, m, s, x }
    }
}

impl From<String> for Part {
    fn from(value: String) -> Self {
        let value = value[1..value.len() - 1].to_string();
        let toks = value.split(',');
        let mut a = 0;
        let mut m = 0;
        let mut s = 0;
        let mut x = 0;
        for tok in toks {
            let prop_val = tok.split('=').collect::<Vec<&str>>();
            let prop = prop_val[0];
            let val = prop_val[1].parse::<i64>().unwrap();
            match prop {
                "a" => a = val,
                "m" => m = val,
                "s" => s = val,
                "x" => x = val,
                _ => panic!("Unknown property {}", prop),
            }
        }
        Self::new(a, m, s, x)
    }
}

#[derive(Debug, Clone)]
struct Workflow {
    rules: Vec<Rule>,
    name: String,
}

impl Workflow {
    fn apply(&self, part: &Part) -> Option<String> {
        for rule in self.rules.iter() {
            if let Some(action) = rule.apply(part) {
                return Some(action);
            }
        }
        return None;
    }
}

impl From<String> for Workflow {
    fn from(value: String) -> Self {
        let mut name = String::new();
        let mut rules = Vec::new();
        let mut i = 0;
        for c in value.chars() {
            if c == '{' {
                break;
            }
            name.push(c);
            i += 1;
        }
        let rule_toks = value[i + 1..value.len() - 1].split(',');
        for tok in rule_toks {
            if !tok.contains(':') {
                rules.push(Rule {
                    prop: char::default(),
                    cond_type: char::default(),
                    cond_val: 0,
                    action: tok.to_string(),
                    neg: false,
                });
                continue;
            }
            let rule = tok.split(':').collect::<Vec<&str>>();
            let cond = rule[0];
            let action = rule[1];
            let prop = cond.chars().nth(0).unwrap();
            let cond_type = cond.chars().nth(1).unwrap();
            let cond_val = cond[2..cond.len()].parse::<i64>().unwrap();
            rules.push(Rule {
                prop,
                cond_type,
                cond_val,
                action: action.to_string(),
                neg: false,
            });
        }
        Self { rules, name }
    }
}

#[derive(Debug, Clone)]
struct Rule {
    prop: char,
    cond_type: char,
    cond_val: i64,
    action: String,
    neg: bool,
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cond_type = match self.cond_type {
            '<' => {
                if self.neg {
                    ">="
                } else {
                    "<"
                }
            }
            '>' => {
                if self.neg {
                    "<="
                } else {
                    ">"
                }
            }
            _ => "",
        };
        write!(
            f,
            "{}{}{} => {}",
            self.prop, cond_type, self.cond_val, self.action
        )
    }
}

impl Rule {
    fn negate(&self) -> Self {
        let mut rule = self.clone();
        rule.neg = !rule.neg;
        rule
    }

    fn apply(&self, part: &Part) -> Option<String> {
        let action = self.action.clone();
        match (self.prop, self.cond_type) {
            ('a', '<') => {
                if part.a < self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('a', '>') => {
                if part.a > self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('m', '<') => {
                if part.m < self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('m', '>') => {
                if part.m > self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('s', '<') => {
                if part.s < self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('s', '>') => {
                if part.s > self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('x', '<') => {
                if part.x < self.cond_val {
                    return Some(action);
                }
                return None;
            }
            ('x', '>') => {
                if part.x > self.cond_val {
                    return Some(action);
                }
                return None;
            }
            _ => return Some(action),
        }
    }
}

fn get_contents(filename: &str) -> (Vec<Workflow>, Vec<Part>) {
    let lines = read_lines(filename);
    let mut workflows = Vec::new();
    let mut i = 0;
    for line in lines.iter() {
        if line == "" {
            break;
        }
        workflows.push(Workflow::from(line.clone()));
        i += 1;
    }
    let mut parts = Vec::new();
    for line in lines[i + 1..].iter() {
        parts.push(Part::from(line.to_string()));
    }
    return (workflows, parts);
}

fn part1((workflows, parts): &(Vec<Workflow>, Vec<Part>)) -> i64 {
    let mut workflow_map = HashMap::new();
    let mut accepted = Vec::new();
    for workflow in workflows {
        workflow_map.insert(workflow.name.clone(), workflow);
    }
    for part in parts.iter() {
        let mut workflow = workflow_map.get("in").unwrap();
        while let Some(action) = workflow.apply(part) {
            if action == "A" {
                accepted.push(part);
                break;
            } else if action == "R" {
                break;
            } else {
                workflow = workflow_map.get(&action).unwrap();
            }
        }
    }
    let mut sum = 0;
    for part in accepted.iter() {
        sum += part.a;
        sum += part.m;
        sum += part.s;
        sum += part.x;
    }
    return sum;
}

fn rule_bounds(
    rule: &Rule,
    min_a: &mut i64,
    max_a: &mut i64,
    min_m: &mut i64,
    max_m: &mut i64,
    min_s: &mut i64,
    max_s: &mut i64,
    min_x: &mut i64,
    max_x: &mut i64,
    invalid: &mut bool,
) -> () {
    if !rule.neg {
        match (rule.prop, rule.cond_type, rule.cond_val) {
            ('a', '<', val) => {
                *max_a = val - 1;
                if max_a < min_a {
                    *invalid = true;
                }
            }
            ('a', '>', val) => {
                *min_a = val + 1;
                if min_a > max_a {
                    *invalid = true;
                }
            }
            ('m', '<', val) => {
                *max_m = val - 1;
                if max_m < min_m {
                    *invalid = true;
                }
            }
            ('m', '>', val) => {
                *min_m = val + 1;
                if min_m > max_m {
                    *invalid = true;
                }
            }
            ('s', '<', val) => {
                *max_s = val - 1;
                if max_s < min_s {
                    *invalid = true;
                }
            }
            ('s', '>', val) => {
                *min_s = val + 1;
                if min_s > max_s {
                    *invalid = true;
                }
            }
            ('x', '<', val) => {
                *max_x = val - 1;
                if max_x < min_x {
                    *invalid = true;
                }
            }
            ('x', '>', val) => {
                *min_x = val + 1;
                if min_x > max_x {
                    *invalid = true;
                }
            }
            _ => {}
        }
    } else {
        match (rule.prop, rule.cond_type, rule.cond_val) {
            ('a', '<', val) => {
                *min_a = val;
                if min_a > max_a {
                    *invalid = true;
                }
            }
            ('a', '>', val) => {
                *max_a = val;
                if max_a < min_a {
                    *invalid = true;
                }
            }
            ('m', '<', val) => {
                *min_m = val;
                if min_m > max_m {
                    *invalid = true;
                }
            }
            ('m', '>', val) => {
                *max_m = val;
                if max_m < min_m {
                    *invalid = true;
                }
            }
            ('s', '<', val) => {
                *min_s = val;
                if min_s > max_s {
                    *invalid = true;
                }
            }
            ('s', '>', val) => {
                *max_s = val;
                if max_s < min_s {
                    *invalid = true;
                }
            }
            ('x', '<', val) => {
                *min_x = val;
                if min_x > max_x {
                    *invalid = true;
                }
            }
            ('x', '>', val) => {
                *max_x = val;
                if max_x < min_x {
                    *invalid = true;
                }
            }
            _ => {}
        }
    }
}

fn count_possibilities(
    src: &String,
    workflow_map: &HashMap<String, Workflow>,
    a_min: i64,
    a_max: i64,
    m_min: i64,
    m_max: i64,
    s_min: i64,
    s_max: i64,
    x_min: i64,
    x_max: i64,
) -> i64 {
    if src == "A" {
        return (a_max - a_min + 1)
            * (m_max - m_min + 1)
            * (s_max - s_min + 1)
            * (x_max - x_min + 1);
    } else if src == "R" {
        return 0;
    } else {
        let mut total = 0;
        let workflow = workflow_map.get(src).unwrap();
        for (i, rule) in workflow.rules.iter().enumerate() {
            let mut min_a = a_min;
            let mut max_a = a_max;
            let mut min_m = m_min;
            let mut max_m = m_max;
            let mut min_s = s_min;
            let mut max_s = s_max;
            let mut min_x = x_min;
            let mut max_x = x_max;
            let mut invalid = false;

            workflow.rules[0..i].iter().for_each(|r| {
                rule_bounds(
                    &r.negate(),
                    &mut min_a,
                    &mut max_a,
                    &mut min_m,
                    &mut max_m,
                    &mut min_s,
                    &mut max_s,
                    &mut min_x,
                    &mut max_x,
                    &mut invalid,
                );
            });
            rule_bounds(
                rule,
                &mut min_a,
                &mut max_a,
                &mut min_m,
                &mut max_m,
                &mut min_s,
                &mut max_s,
                &mut min_x,
                &mut max_x,
                &mut invalid,
            );

            total += if invalid {
                0
            } else {
                count_possibilities(
                    &rule.action,
                    workflow_map,
                    min_a,
                    max_a,
                    min_m,
                    max_m,
                    min_s,
                    max_s,
                    min_x,
                    max_x,
                )
            };
        }
        return total;
    }
}

fn part2((workflows, _): &(Vec<Workflow>, Vec<Part>)) -> i64 {
    let mut workflow_map = HashMap::new();
    for workflow in workflows.iter() {
        let w = workflow.clone();
        workflow_map.insert(workflow.name.clone(), w);
    }
    count_possibilities(
        &"in".to_string(),
        &workflow_map,
        1,
        4000,
        1,
        4000,
        1,
        4000,
        1,
        4000,
    )
}

test!(
    part1 {
        "test_inputs/day19/test01.txt" => 19114
    },
    part2 {
        "test_inputs/day19/test01.txt" => 167409079868000
    }

);
main!();
