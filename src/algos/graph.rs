use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::Add,
};

use super::{priority_queue, traits::MinMax};

#[derive(Debug, Clone)]
pub struct AdjacencyList<T, W> {
    pub nodes: Vec<T>,
    pub edges: Vec<Vec<(usize, W)>>,
}

pub trait Graph<T, W> {
    fn add_node(&mut self, value: T) -> usize;
    fn add_weighted_edge(&mut self, from: usize, to: usize, weight: W);
    fn node(&self, index: usize) -> AdjNode<T, W>;
    fn node_mut(&mut self, index: usize) -> AdjNodeMut<T, W>;
    fn iter(&self) -> AdjNodeIter<T, W>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

pub struct AdjNode<'a, T, W> {
    index: usize,
    container: &'a AdjacencyList<T, W>,
}

pub struct AdjNodeMut<'a, T, W> {
    index: usize,
    container: &'a mut AdjacencyList<T, W>,
}
pub struct AdjNodeIter<'a, T, W> {
    index: usize,
    container: &'a AdjacencyList<T, W>,
}

impl<'a, T, W> Iterator for AdjNodeIter<'a, T, W> {
    type Item = AdjNode<'a, T, W>;

    fn next(&mut self) -> Option<Self::Item> {
        let container = &self.container;
        if self.index >= container.nodes.len() {
            return None;
        }
        let index = self.index;
        self.index += 1;
        let next = AdjNode::new(index, self.container);
        return Some(next);
    }
}

pub struct AdjNodeNeighborsIter<'a, T, W> {
    node: AdjNode<'a, T, W>,
    index: usize,
}

impl<'a, T, W> Iterator for AdjNodeNeighborsIter<'a, T, W> {
    type Item = AdjNode<'a, T, W>;

    fn next(&mut self) -> Option<Self::Item> {
        let container = &self.node.container;
        if self.index >= container.edges[self.node.index].len() {
            return None;
        }
        let (index, _) = container.edges[self.node.index][self.index];
        self.index += 1;
        let next = AdjNode::new(index, self.node.container);
        return Some(next);
    }
}

impl<'a, T, W> AdjNode<'a, T, W> {
    fn new(index: usize, container: &'a AdjacencyList<T, W>) -> Self {
        return Self { index, container };
    }

    pub fn neighbors(&self) -> AdjNodeNeighborsIter<T, W> {
        return AdjNodeNeighborsIter {
            node: AdjNode::new(self.index, self.container),
            index: 0,
        };
    }

    pub fn value(&self) -> &T {
        return &self.container.nodes[self.index];
    }

    pub fn index(&self) -> usize {
        return self.index;
    }
}

impl<'a, T, W> AdjNodeMut<'a, T, W> {
    fn new(index: usize, container: &'a mut AdjacencyList<T, W>) -> Self {
        return Self { index, container };
    }

    pub fn neighbors(&self) -> AdjNodeNeighborsIter<T, W> {
        return AdjNodeNeighborsIter {
            node: AdjNode::new(self.index, self.container),
            index: 0,
        };
    }

    pub fn value(&mut self) -> &mut T {
        return &mut self.container.nodes[self.index];
    }

    pub fn value_mut(&mut self) -> &mut T {
        return &mut self.container.nodes[self.index];
    }
}

impl<T, W: Clone> AdjacencyList<T, W> {
    pub fn new() -> Self {
        return Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        };
    }

    fn edges_iter(&self) -> impl Iterator<Item = (usize, usize, W)> + '_ {
        return self
            .edges
            .iter()
            .enumerate()
            .flat_map(|(from, edges)| edges.iter().map(move |&(to, ref w)| (from, to, w.clone())));
    }
}

impl<T, W: Clone> Graph<T, W> for AdjacencyList<T, W> {
    fn add_node(&mut self, value: T) -> usize {
        let index = self.nodes.len();
        self.nodes.push(value);
        self.edges.push(Vec::new());
        return index;
    }

    fn add_weighted_edge(&mut self, from: usize, to: usize, weight: W) {
        self.edges[from].push((to, weight));
    }

    fn node<'a>(&'a self, index: usize) -> AdjNode<'a, T, W> {
        return AdjNode::new(index, self);
    }

    fn node_mut<'a>(&'a mut self, index: usize) -> AdjNodeMut<'a, T, W> {
        return AdjNodeMut::new(index, self);
    }

    fn iter(&self) -> AdjNodeIter<T, W> {
        return AdjNodeIter {
            index: 0,
            container: self,
        };
    }

    fn len(&self) -> usize {
        return self.nodes.len();
    }

    fn is_empty(&self) -> bool {
        return self.nodes.is_empty();
    }

    fn clear(&mut self) {
        self.nodes.clear();
        self.edges.clear();
    }
}

impl<T: Clone, W: Clone> AdjacencyList<T, W> {
    pub fn clone_from(&mut self, other: &Self) {
        self.nodes.clone_from(&other.nodes);
        self.edges.clone_from(&other.edges);
    }
}

impl<T, W: Clone + Hash + Eq> AdjacencyList<T, W> {
    pub fn bfs_mut<F>(&mut self, start: usize, mut f: F)
    where
        F: FnMut(&mut T),
    {
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            f(&mut self.nodes[node]);
            for neighbor in self.node(node).neighbors() {
                queue.push_back(neighbor.index);
            }
        }
    }

    pub fn bfs<F>(&mut self, start: usize, f: F)
    where
        F: Fn(&T),
    {
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();
        queue.push_back(start);
        while let Some(node) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            f(&self.nodes[node]);
            for neighbor in self.node(node).neighbors() {
                queue.push_back(neighbor.index);
            }
        }
    }

    pub fn dfs_mut<F>(&mut self, start: usize, mut f: F)
    where
        F: FnMut(&mut T),
    {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = Vec::new();
        stack.push(start);
        while let Some(node) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            f(&mut self.nodes[node]);
            for neighbor in self.node(node).neighbors() {
                stack.push(neighbor.index);
            }
        }
    }

    pub fn dfs<F>(&mut self, start: usize, f: F)
    where
        F: Fn(&T),
    {
        let mut visited = vec![false; self.nodes.len()];
        let mut stack = Vec::new();
        stack.push(start);
        while let Some(node) = stack.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            f(&self.nodes[node]);
            for neighbor in self.node(node).neighbors() {
                stack.push(neighbor.index);
            }
        }
    }

    pub fn is_connected(&self) -> bool {
        if self.nodes.is_empty() {
            return true;
        }
        let mut visited = vec![false; self.nodes.len()];
        let mut queue = VecDeque::new();
        queue.push_back(0);
        while let Some(node) = queue.pop_front() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            for neighbor in self.node(node).neighbors() {
                queue.push_back(neighbor.index);
            }
        }
        return visited.iter().all(|&v| v);
    }

    pub fn connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.nodes.len()];
        let mut components = Vec::new();
        for node in 0..self.nodes.len() {
            if visited[node] {
                continue;
            }
            let mut component = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back(node);
            while let Some(node) = queue.pop_front() {
                if visited[node] {
                    continue;
                }
                visited[node] = true;
                component.push(node);
                for neighbor in self.node(node).neighbors() {
                    queue.push_back(neighbor.index);
                }
            }
            components.push(component);
        }
        return components;
    }

    pub fn topological_sort(&self) -> Option<Vec<usize>> {
        // kahn's algorithm
        let mut in_degrees = vec![0; self.nodes.len()];
        let mut edges = self
            .edges_iter()
            .map(|(from, to, _)| (from, to))
            .collect::<HashSet<_>>();
        edges.iter().for_each(|&(_, to)| in_degrees[to] += 1);
        let mut s = in_degrees
            .iter()
            .enumerate()
            .filter_map(|(i, &d)| if d == 0 { Some(i) } else { None })
            .collect::<VecDeque<_>>();
        let mut sorted = Vec::new();
        while let Some(node) = s.pop_front() {
            sorted.push(node);
            for neighbor in self.node(node).neighbors() {
                in_degrees[neighbor.index] -= 1;
                edges.remove(&(node, neighbor.index));
                if in_degrees[neighbor.index] == 0 {
                    s.push_back(neighbor.index);
                }
            }
        }
        if !edges.is_empty() {
            return None;
        }
        return Some(sorted);
    }

    pub fn is_dag(&self) -> bool {
        return self.topological_sort().is_some();
    }
}

impl<T: Default + Clone, W: Clone + Default> AdjacencyList<T, W> {
    pub fn with(num_nodes: usize) -> Self {
        return AdjacencyList {
            nodes: vec![T::default(); num_nodes],
            edges: vec![Vec::new(); num_nodes],
        };
    }

    pub fn fill(&mut self, value: T) {
        self.nodes.fill(value);
    }

    pub fn with_edges(edges: &[(usize, usize)]) -> Self {
        let num_nodes = edges
            .iter()
            .map(|&(from, to)| std::cmp::max(from, to))
            .max()
            .unwrap_or(0)
            + 1;
        let mut adj = Self::with(num_nodes);
        for &(from, to) in edges {
            adj.add_weighted_edge(from, to, W::default());
        }
        return adj;
    }
}

#[derive(Debug, Clone)]
pub struct Empty;

impl Default for Empty {
    fn default() -> Self {
        return Self {};
    }
}

impl<T: Clone + Default, W: Clone + Default> Default for AdjacencyList<T, W> {
    fn default() -> Self {
        return Self::new();
    }
}

impl<T: Clone + Default> From<&[(usize, usize)]> for AdjacencyList<T, Empty> {
    fn from(edges: &[(usize, usize)]) -> Self {
        return Self::with_edges(edges);
    }
}

impl<T: Clone + Default, W: Clone + Default> From<&[(usize, usize, W)]> for AdjacencyList<T, W> {
    fn from(edges: &[(usize, usize, W)]) -> Self {
        let num_nodes = edges
            .iter()
            .map(|&(from, to, _)| std::cmp::max(from, to))
            .max()
            .unwrap_or(0)
            + 1;
        let mut adj = Self::with(num_nodes);
        for &(from, to, ref w) in edges {
            adj.add_weighted_edge(from, to, w.clone());
        }
        return adj;
    }
}

impl<T: Clone + Default, W: Clone + Default + MinMax + Add<Output = W> + Ord + Eq>
    AdjacencyList<T, W>
{
    const INF: W = W::MAX;

    pub fn dijkstra(&self, start: usize) -> Vec<Option<W>> {
        let mut dist = vec![None; self.nodes.len()];
        let mut visited = vec![false; self.nodes.len()];
        dist[start] = Some(W::default());
        let mut queue = priority_queue::BinaryHeap::new();
        queue.push(start, W::default());
        while let Some((node, _)) = queue.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            if dist[node].is_none() {
                eprintln!("dist[node] == INF");
                continue;
            }
            for &(neighbor, ref weight) in self.edges[node].iter() {
                let Some(curr_dist) = dist[node].clone() else {
                    continue;
                };
                if curr_dist.clone() + weight.clone() < dist[neighbor].clone().unwrap_or(Self::INF)
                {
                    dist[neighbor] = Some(curr_dist.clone() + weight.clone());
                    queue.push(neighbor, dist[neighbor].clone().unwrap_or(Self::INF));
                }
            }
        }
        return dist;
    }

    pub fn dijkstra_with_path(&self, start: usize) -> (Vec<W>, Vec<Option<usize>>) {
        let mut dist = vec![Self::INF; self.nodes.len()];
        let mut visited = vec![false; self.nodes.len()];
        let mut parent = vec![None; self.nodes.len()];
        dist[start] = W::default();
        let mut queue = priority_queue::BinaryHeap::new();
        queue.push(start, W::default());
        while let Some((node, _)) = queue.pop() {
            if visited[node] {
                continue;
            }
            visited[node] = true;
            for &(neighbor, ref weight) in self.edges[node].iter() {
                if dist[node] == Self::INF {
                    eprintln!("dist[node] == INF");
                    continue;
                }
                if dist[node].clone() + weight.clone() < dist[neighbor] {
                    dist[neighbor] = dist[node].clone() + weight.clone();
                    parent[neighbor] = Some(node);
                    queue.push(neighbor, dist[neighbor].clone());
                }
            }
        }
        return (dist, parent);
    }

    pub fn bellman_ford(&self, start: usize) -> Option<Vec<W>> {
        let mut dist = vec![Self::INF; self.nodes.len()];
        dist[start] = W::default();
        for _ in 0..self.nodes.len() - 1 {
            for from in 0..self.nodes.len() {
                for &(to, ref weight) in self.edges[from].iter() {
                    if dist[from].clone() + weight.clone() < dist[to].clone() {
                        dist[to] = dist[from].clone() + weight.clone();
                    }
                }
            }
        }
        for from in 0..self.nodes.len() {
            for &(to, ref weight) in self.edges[from].iter() {
                if dist[from].clone() + weight.clone() < dist[to] {
                    return None;
                }
            }
        }
        return Some(dist);
    }

    pub fn floyd_warshall(&self) -> Vec<Vec<W>> {
        let mut dist = vec![vec![Self::INF; self.nodes.len()]; self.nodes.len()];
        for from in 0..self.nodes.len() {
            dist[from][from] = W::default();
            for &(to, ref weight) in self.edges[from].iter() {
                dist[from][to] = weight.clone();
            }
        }

        for k in 0..self.nodes.len() {
            for i in 0..self.nodes.len() {
                for j in 0..self.nodes.len() {
                    if dist[i][k] != Self::INF
                        && dist[k][j] != Self::INF
                        && dist[i][k].clone() + dist[k][j].clone() < dist[i][j].clone()
                    {
                        dist[i][j] = dist[i][k].clone() + dist[k][j].clone();
                    }
                }
            }
        }
        return dist;
    }
}
