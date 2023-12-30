// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use std::collections::HashMap;

/// A graph with unidirectional edges
pub struct UniGraph<NT, ET, Idx = u8, CT = u32>
where
    Idx: TryFrom<usize> + std::fmt::Debug,
    NT: Clone,
    ET: Clone,
{
    pub nodes: Vec<(String, NT)>,
    pub edges: HashMap<Idx, Vec<(Idx, CT, ET)>>,
}

/// A graph with bidirectional edges
pub struct BiGraph<NT, ET, Idx = u8, CT = u32>
where
    Idx: TryFrom<usize> + std::fmt::Debug,
    NT: Clone,
    ET: Clone,
{
    pub nodes: Vec<(String, NT)>,
    pub edges: HashMap<Idx, Vec<(Idx, CT, ET)>>,
}

impl<NT, ET, Idx, CT> UniGraph<NT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + std::fmt::Debug + Eq + std::hash::Hash,
    <Idx as TryFrom<usize>>::Error: std::fmt::Debug,
    NT: Clone,
    ET: Clone,
{
    pub fn new<CF>(cost_func: CF, data: &[(&str, NT, Vec<(&str, ET)>)]) -> Self
    where
        CF: Fn(&NT, &ET) -> CT,
    {
        let mut nodes = Vec::new();
        let mut edges = HashMap::new();
        let mut nodename_to_idx = HashMap::new();

        for (node, node_data, _) in data {
            nodename_to_idx.insert(node, nodes.len());
            edges.insert(
                Idx::try_from(nodes.len()).expect("Node# fits Idx"),
                Vec::new(),
            );
            nodes.push((node.to_string(), node_data.clone()));
        }

        for (node, _, node_edges) in data {
            let node = nodename_to_idx.get(node).expect("Node exists");
            for (edge, edge_data) in node_edges {
                let edge = nodename_to_idx.get(edge).expect("edge destination exists");
                let edges = edges
                    .get_mut(&Idx::try_from(*node).expect("Idx fits"))
                    .expect("Node exists");
                edges.push((
                    Idx::try_from(*edge).expect("Fits"),
                    cost_func(&nodes[*node].1, edge_data),
                    edge_data.clone(),
                ));
            }
        }

        UniGraph { nodes, edges }
    }
}

impl<NT, ET, Idx, CT> BiGraph<NT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + std::fmt::Debug + Eq + std::hash::Hash,
    <Idx as TryFrom<usize>>::Error: std::fmt::Debug,
    NT: Clone,
    ET: Clone,
{
    pub fn compress<F>(unigraph: UniGraph<NT, ET, Idx, CT>, should_combine: F) -> Self
    where
        F: Fn(&NT, &NT, &NT, &ET, CT, &ET, CT) -> Option<(CT, ET)>,
        Idx: Into<usize> + Copy,
        CT: Copy,
    {
        let mut graph = BiGraph {
            nodes: unigraph.nodes,
            edges: HashMap::new(),
        };

        // Add edges in both directions
        for (from_node, old_list) in unigraph.edges {
            for (node2, cost, edge_data) in &old_list {
                let list2 = graph.edges.entry(from_node).or_insert_with(Vec::new);
                if !list2.iter().any(|(node, _, _)| node == node2) {
                    list2.push((*node2, *cost, edge_data.clone()));
                }
                let list2 = graph.edges.entry(*node2).or_insert_with(Vec::new);
                if !list2.iter().any(|(node, _, _)| *node == from_node) {
                    list2.push((from_node, *cost, edge_data.clone()));
                }
            }
        }

        loop {
            let mut any_change = false;

            // HashMap<Idx, Vec<(Idx, CT, &'a ET)>>,
            let mut update = None;
            'find_edge: for (node1, edges) in &graph.edges {
                if edges.len() != 2 {
                    continue;
                }
                for (node2, cost, edge_data) in edges {
                    for (node3, cost3, edge_data3) in edges {
                        if node3 == node2 {
                            continue;
                        }
                        if let Some((cost, data)) = should_combine(
                            &graph.nodes[Idx::into(*node1)].1,
                            &graph.nodes[Idx::into(*node2)].1,
                            &graph.nodes[Idx::into(*node3)].1,
                            edge_data,
                            *cost,
                            edge_data3,
                            *cost3,
                        ) {
                            update = Some((*node1, *node2, *node3, (cost, data)));
                            any_change = true;
                            break 'find_edge;
                        }
                    }
                }
            }
            if let Some((node_to_remove, node1, node2, cost_and_data)) = update {
                graph.edges.remove(&node_to_remove);

                // Update edges at node1 from node_to_remove to node2
                let list = graph.edges.get_mut(&node1).expect("edge list");
                for (node, cost, data) in list.iter_mut() {
                    if *node == node_to_remove {
                        *node = node2;
                        *cost = cost_and_data.0;
                        *data = cost_and_data.1.clone();
                    }
                }
                // Update edges at node2 from node_to_remove to node1
                let list = graph.edges.get_mut(&node2).expect("edge list");
                for (node, cost, data) in list.iter_mut() {
                    if *node == node_to_remove {
                        *node = node1;
                        *cost = cost_and_data.0;
                        *data = cost_and_data.1.clone();
                    }
                }
            }

            if !any_change {
                break;
            }
        }
        graph
    }
}

impl<NT, ET, Idx, CT> std::fmt::Debug for UniGraph<NT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + Into<usize> + std::fmt::Debug + Copy,
    NT: std::fmt::Debug + Clone,
    ET: std::fmt::Debug + Clone,
    CT: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "UniGraph {{")?;
        for (from_node, list) in &self.edges {
            for (node, cost, data) in list {
                writeln!(
                    fmt,
                    "  {} -({:?}, {:?})> {}",
                    self.nodes[(*from_node).into()].0,
                    cost,
                    data,
                    self.nodes[(*node).into()].0
                )?;
            }
        }
        writeln!(fmt, "}}")
    }
}

impl<NT, ET, Idx, CT> std::fmt::Debug for BiGraph<NT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + Into<usize> + std::fmt::Debug + Copy,
    NT: std::fmt::Debug + Clone,
    ET: std::fmt::Debug + Clone,
    CT: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "BiGraph {{")?;
        for (from_node, list) in &self.edges {
            for (node, cost, data) in list {
                writeln!(
                    fmt,
                    "  {} -({:?}, {:?})> {}",
                    self.nodes[(*from_node).into()].0,
                    cost,
                    data,
                    self.nodes[(*node).into()].0
                )?;
            }
        }
        writeln!(fmt, "}}")
    }
}
