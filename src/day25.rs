use crate::utils::{main, read_lines, test};

use crate::algos::priority_queue::BinaryHeap;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

type InputData = HashMap<String, HashSet<String>>;

fn get_contents(filename: &str) -> InputData {
    let lines = read_lines(filename);
    let mut connections = lines
        .iter()
        .map(|line| {
            let mut toks = line.split(":");
            let name = toks.next().unwrap().to_string();
            let mut nbr = HashSet::new();
            let children_str = toks.next().unwrap().trim();
            for nbr_name in children_str.split_whitespace() {
                nbr.insert(nbr_name.trim().to_string());
            }
            (name, nbr)
        })
        .collect::<HashMap<_, _>>();
    for name in connections.keys().cloned().collect::<Vec<_>>() {
        let nbr = connections.get(&name).unwrap().clone();
        for nbr_name in nbr.iter() {
            connections
                .entry(nbr_name.to_string())
                .or_insert(HashSet::new())
                .insert(name.clone());
        }
    }
    return connections;
}

fn merge_nodes(
    graph: &mut InputData,
    w: &mut HashMap<(String, String), usize>,
    a: &String,
    b: &String,
) {
    if let Some(nbrs_a) = graph.get_mut(a) {
        nbrs_a.remove(b);
    } else {
        panic!();
    }
    if let Some(nbrs_b) = graph.get_mut(b) {
        nbrs_b.remove(a);
    } else {
        panic!();
    }
    let nbrs_a = graph.get(a).unwrap().clone();
    let nbrs_b = graph.get(b).unwrap().clone();
    let nbrs = nbrs_a.union(&nbrs_b).cloned().collect::<Vec<_>>();
    let mut new_node = String::new();

    if a < b {
        new_node.push_str(a.as_str());
        new_node.push_str(b.as_str());
    } else {
        new_node.push_str(b.as_str());
        new_node.push_str(a.as_str());
    }
    for nbr in nbrs_a.iter() {
        let Some(nbrs) = graph.get_mut(nbr) else {
            panic!();
        };
        nbrs.remove(a);
        nbrs.insert(new_node.clone());
    }
    for nbr in nbrs_b.iter() {
        let Some(nbrs) = graph.get_mut(nbr) else {
            panic!();
        };
        nbrs.remove(b);
        nbrs.insert(new_node.clone());
    }

    for nbr in nbrs.iter() {
        let nbr_a = w.get(&(a.clone(), nbr.clone())).unwrap_or(&0).clone();
        let nbr_b = w.get(&(b.clone(), nbr.clone())).unwrap_or(&0).clone();
        let b_nbr = w.get(&(nbr.clone(), b.clone())).unwrap_or(&0).clone();
        let a_nbr = w.get(&(nbr.clone(), a.clone())).unwrap_or(&0).clone();

        let weight = (nbr_a + nbr_b + b_nbr + a_nbr) / 2;

        w.insert((new_node.clone(), nbr.clone()), weight);
        w.insert((nbr.clone(), new_node.clone()), weight);
        w.remove(&(a.clone(), nbr.clone()));
        w.remove(&(nbr.clone(), a.clone()));
    }

    graph.remove(a);
    graph.remove(b);
    graph.insert(new_node.clone(), nbrs.into_iter().collect::<HashSet<_>>());
}

fn distance(set: &Vec<String>, w: &HashMap<(String, String), usize>, a: &String) -> usize {
    let mut dist = 0;
    for node in set.iter() {
        dist += w.get(&(a.clone(), node.clone())).unwrap_or(&0);
    }
    return dist;
}

#[allow(non_snake_case)]
fn min_cut_phase(
    graph: &mut InputData,
    w: &mut HashMap<(String, String), usize>,
    a: &String,
) -> (String, usize) {
    let mut A = Vec::new();
    A.push(a.clone());
    let mut B = graph.keys().cloned().collect::<HashSet<_>>();
    B.remove(a);
    let mut cut = 0;
    let mut heap = BinaryHeap::new();
    for node in B.iter() {
        let dist = distance(&A, &w, node);
        heap.push(node.clone(), Reverse(dist));
    }

    while A.len() != graph.len() {
        let best = heap.pop().unwrap();
        let nbrs = graph.get(&best.0).unwrap();
        for nbr in nbrs.iter() {
            if B.contains(nbr) {
                let dist = distance(&A, &w, nbr);
                heap.decrease_priority(nbr, Reverse(dist));
            }
        }
        B.remove(&best.0);
        A.push(best.0);
        cut = best.1 .0;
    }
    let last = A.pop().unwrap();
    let last2 = A.pop().unwrap();
    merge_nodes(graph, w, &last, &last2);
    return (last + &last2, cut);
}

fn stoer_wagner(graph: &InputData) -> usize {
    let mut cut_sizes = Vec::new();
    let mut cut_nodes = Vec::new();
    let a = graph.keys().next().unwrap();
    let n_nodes = graph.len();
    let mut graph = graph.clone();
    let mut w = HashMap::new();
    for (node, nbrs) in graph.iter() {
        for nbr in nbrs.iter() {
            w.insert((node.clone(), nbr.clone()), 1);
            w.insert((nbr.clone(), node.clone()), 1);
        }
    }
    while graph.len() > 1 {
        let (cut_node, cut_size) = min_cut_phase(&mut graph, &mut w, a);
        cut_sizes.push(cut_size);
        cut_nodes.push(cut_node);
    }
    let (pos, _) = cut_sizes
        .iter()
        .enumerate()
        .min_by_key(|(_, cut)| *cut)
        .unwrap();
    let min_cut_node = cut_nodes.get(pos).unwrap();
    let partition_size = min_cut_node.len() / 3;
    return partition_size * (n_nodes - partition_size);
}

fn part1(connections: &InputData) -> i64 {
    stoer_wagner(connections) as i64
}

fn part2(_input_file: &InputData) -> i64 {
    0
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
