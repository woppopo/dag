use std::collections::HashSet;

pub struct DirectedAcyclicGraph {
    nodes: HashSet<usize>,
    edges: HashSet<(usize, usize)>,
    used: usize,
}

impl DirectedAcyclicGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashSet::new(),
            used: 0,
        }
    }

    pub fn add_node(&mut self) -> usize {
        let id = self.used;
        self.nodes.insert(id);
        self.used += 1;
        id
    }

    pub fn has_node(&self, id: usize) -> bool {
        self.nodes.contains(&id)
    }

    pub fn remove_node(&mut self, id: usize) {
        self.nodes.remove(&id);
        self.edges.retain(|&(a, b)| a != id && b != id);
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> bool {
        if self.is_reachable(to, from) {
            return false;
        }

        self.edges.insert((from, to));
        true
    }

    pub fn has_edge(&self, from: usize, to: usize) -> bool {
        self.edges.contains(&(from, to))
    }

    pub fn remove_edge(&mut self, from: usize, to: usize) {
        self.edges.retain(|&(a, b)| a != from && b != to);
    }

    pub fn is_reachable(&self, from: usize, to: usize) -> bool {
        let mut current_nodes: HashSet<_> = std::iter::once(from).collect();
        while !current_nodes.is_empty() {
            if current_nodes.contains(&to) {
                return true;
            }

            current_nodes = current_nodes
                .into_iter()
                .flat_map(|to| self.outgoing_nodes(to))
                .collect();
        }
        false
    }

    pub fn outgoing_nodes(&self, from: usize) -> HashSet<usize> {
        self.edges
            .iter()
            .filter(|&&(a, _)| a == from)
            .map(|&(_, b)| b)
            .collect()
    }

    pub fn ingoing_nodes(&self, to: usize) -> HashSet<usize> {
        self.edges
            .iter()
            .filter(|&&(_, b)| b == to)
            .map(|&(a, _)| a)
            .collect()
    }

    pub fn topological_sort(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut current_nodes: HashSet<_> = self.nodes.clone();
        while !current_nodes.is_empty() {
            let is_there_no_ingoing = |id| {
                let ingoing = self.ingoing_nodes(id);
                ingoing.intersection(&current_nodes).next().is_none() // ingoing.is_empty()
            };

            let removable: HashSet<_> = current_nodes
                .iter()
                .filter(|&&id| is_there_no_ingoing(id))
                .cloned()
                .collect();

            for id in removable.into_iter() {
                current_nodes.remove(&id);
                result.push(id);
            }
        }
        result
    }
}
