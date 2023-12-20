use std::collections::{HashMap, VecDeque};

use crate::{
    algos::{
        graph::{self, AdjacencyList, Graph},
        lcm,
    },
    utils::{main, read_lines, test},
};
use std::io::{self, Write};

type InputData = (AdjacencyList<String, ()>, HashMap<String, Module>);

#[derive(Clone)]
struct FlipFlop {
    state: bool,
}

impl FlipFlop {
    fn new() -> Self {
        Self { state: false }
    }

    fn output(&mut self, input: bool) -> bool {
        let out = if !input {
            self.state = !self.state;
            self.state
        } else {
            self.state
        };
        out
    }
}

#[derive(Clone)]
struct Conjunction {
    saved_states: Vec<bool>,
}

impl Conjunction {
    fn new() -> Self {
        Self {
            saved_states: Vec::new(),
        }
    }

    fn output(&mut self, input: usize, val: bool) -> bool {
        self.saved_states[input] = val;
        let out = !self.saved_states.iter().all(|&s| s);
        out
    }
}

#[derive(Clone)]
enum Module {
    Broadcaster,
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
}

fn press_button<F: FnMut(usize, bool, usize) -> bool>(
    (graph, modules): &InputData,
    n: i64,
    mut func: F,
) {
    let mut modules = modules.clone();
    let modules_to_node = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect::<HashMap<_, _>>();
    let start_node = modules_to_node.get("broadcaster").unwrap();

    let mut in_degree = vec![0; graph.nodes.len()];
    for edges in graph.edges.iter() {
        for (neighbor, _) in edges.iter() {
            in_degree[*neighbor] += 1;
        }
    }

    let mut processing = vec![false; graph.nodes.len()];
    let mut order = HashMap::<usize, Vec<usize>>::new();
    for node_id in 0..graph.nodes.len() {
        let Some(module) = modules.get_mut(&graph.nodes[node_id]) else {
            continue;
        };
        let Module::Conjunction(conjunction) = module else {
            continue;
        };
        let in_degree = in_degree[node_id];
        conjunction.saved_states = vec![false; in_degree];
    }

    for node_id0 in 0..graph.nodes.len() {
        for node_id1 in 0..graph.nodes.len() {
            let Some(module) = modules.get_mut(&graph.nodes[node_id1]) else {
                continue;
            };
            let Module::Conjunction(_) = module else {
                continue;
            };
            if graph.edges[node_id0].iter().any(|(n, _)| *n == node_id1) {
                if let Some(ord) = order.get_mut(&node_id1) {
                    ord.push(node_id0);
                } else {
                    order.insert(node_id1, vec![node_id0]);
                };
            }
        }
    }

    let mut queue = VecDeque::new();
    let mut button_presses = 0;
    loop {
        button_presses += 1;
        io::stdout().flush().unwrap();
        for (neighbor, _) in graph.edges[*start_node].iter() {
            queue.push_back((*start_node, *neighbor, false));
            processing[*neighbor] = true;
        }

        while processing.iter().any(|&p| p) {
            let Some((sender, receiver, input)) = queue.pop_front() else {
                break;
            };
            let node_name = graph.nodes.get(receiver).unwrap();
            let Some(module) = modules.get_mut(node_name) else {
                continue;
            };
            let output = match module {
                Module::Broadcaster => true,
                Module::FlipFlop(flipflop) => {
                    processing[receiver] = !input;
                    if input {
                        continue;
                    }
                    flipflop.output(input)
                }
                Module::Conjunction(conjunction) => {
                    let Some(pos) = order
                        .get(&receiver)
                        .unwrap()
                        .iter()
                        .position(|&n| n == sender)
                    else {
                        panic!(
                            "{} not found in list of inputs for {}",
                            graph.nodes[sender], graph.nodes[receiver]
                        );
                    };
                    conjunction.output(pos, input)
                }
            };
            if func(receiver, output, button_presses) {
                return;
            }
            for (neighbor, _) in graph.edges[receiver].iter() {
                queue.push_back((receiver, *neighbor, output));
                // let output_name = graph.nodes.get(*neighbor).unwrap();
            }
        }
        if n > 0 && button_presses as i64 >= n {
            return;
        }
    }
}

fn get_contents(filename: &str) -> InputData {
    let mut graph = graph::AdjacencyList::new();
    let lines = read_lines(filename);
    let mut modules = HashMap::new();
    for line in lines.iter() {
        let mut toks = line.split("->");
        let module = toks.next().unwrap().trim();
        let destination = toks
            .next()
            .unwrap()
            .split(", ")
            .map(|s| s.trim())
            .collect::<Vec<_>>();
        let (name, mod_type) = if module == "broadcaster" {
            ("broadcaster", Module::Broadcaster)
        } else if module.starts_with("&") {
            (module[1..].trim(), Module::Conjunction(Conjunction::new()))
        } else if module.starts_with("%") {
            (module[1..].trim(), Module::FlipFlop(FlipFlop::new()))
        } else {
            panic!("Unknown module type: {}", module);
        };

        modules.insert(name.to_string(), mod_type);
        let src = if let Some(pos) = graph.nodes.iter().position(|n| *n == name) {
            pos
        } else {
            graph.add_node(name.to_string())
        };
        for dest in destination {
            let dst = if let Some(pos) = graph.nodes.iter().position(|n| *n == dest) {
                pos
            } else {
                graph.add_node(dest.to_string())
            };
            graph.add_weighted_edge(src, dst, ());
        }
    }
    return (graph, modules);
}

fn part1((graph, modules): &InputData) -> i64 {
    let broadcast_node = graph
        .nodes
        .iter()
        .enumerate()
        .find(|(_, n)| *n == "broadcaster")
        .unwrap()
        .0;

    let n_broadcasts = graph.edges[broadcast_node].len();
    let n_button_presses = 1000;
    let mut nlows = (n_button_presses) * (1 + n_broadcasts);
    let mut nhighs = 0;

    press_button(
        &(graph.clone(), modules.clone()),
        n_button_presses as i64,
        |rec, out, _| {
            let num_nbrs = graph.edges[rec].len();
            if out {
                nhighs += num_nbrs;
            } else {
                nlows += num_nbrs;
            }
            false
        },
    );
    return nlows as i64 * nhighs as i64;
}

fn part2((graph, modules): &InputData) -> i64 {
    let modules_to_node = graph
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect::<HashMap<_, _>>();
    let rx = modules_to_node.get("rx").unwrap();
    let get_parents = |rx: &usize| {
        graph
            .edges
            .iter()
            .enumerate()
            .filter_map(|(sender, receivers)| {
                if receivers.iter().find(|(r, _)| r == rx).is_some() {
                    Some(sender)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    };
    let rx_parents = get_parents(rx);
    let rx_parents = if rx_parents.len() == 1 {
        let rx = rx_parents[0];
        get_parents(&rx)
    } else {
        rx_parents
    };
    rx_parents
        .iter()
        .map(|node| {
            let mut button_presses = 0;
            press_button(
                &(graph.clone(), modules.clone()),
                -1,
                |sender, input, presses| {
                    if sender == *node && input {
                        button_presses = presses;
                        return true;
                    }
                    false
                },
            );
            button_presses as i64
        })
        .fold(1, lcm)
}

#[cfg(test)]
mod consts {
    pub const PART1_INPUTS: [(&str, i64); 0] = [];
    pub const PART2_INPUTS: [(&str, i64); 0] = [];
}

test!();
main!();
