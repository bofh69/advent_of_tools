// SPDX-FileCopyrightText: 2023 Sebastian Andersson <sebastian@bittr.nu>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![warn(missing_docs)]

use std::collections::HashMap;

/// A graph with unidirectional edges
pub struct UniGraph<VT, ET, Idx = u8, CT = u32>
where
    Idx: TryFrom<usize> + std::fmt::Debug,
    VT: Clone,
    ET: Clone,
{
    /// The graph's verices & their data
    pub vertices: Vec<(String, VT)>,
    /// The graph's edges, cost and data
    pub edges: HashMap<Idx, Vec<(Idx, CT, ET)>>,
}

/// A graph with bidirectional edges
pub struct BiGraph<VT, ET, Idx = u8, CT = u32>
where
    Idx: TryFrom<usize> + std::fmt::Debug,
    VT: Clone,
    ET: Clone,
{
    /// The graph's verices & their data
    pub vertices: Vec<(String, VT)>,
    /// The graph's edges, cost and data
    pub edges: HashMap<Idx, Vec<(Idx, CT, ET)>>,
}

/// Data about a vertex and its edges.
///
/// Each vertex has some data attached to it as well as the edge.
type GraphData<'a, VT, ET> = (&'a str, VT, Vec<(&'a str, ET)>);

impl<VT, ET, Idx, CT> UniGraph<VT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + std::fmt::Debug + Eq + std::hash::Hash,
    <Idx as TryFrom<usize>>::Error: std::fmt::Debug,
    VT: Clone,
    ET: Clone,
{
    /// Create a graph from a list of edges & data
    ///
    /// cost_func takes the vertex's data and the edge's to create a cost
    pub fn new<CF>(cost_func: CF, data: &[GraphData<'_, VT, ET>]) -> Self
    where
        CF: Fn(&VT, &ET) -> CT,
    {
        let mut vertices = Vec::new();
        let mut edges = HashMap::new();
        let mut vertexname_to_idx = HashMap::new();

        for (vertex, vertex_data, _) in data {
            vertexname_to_idx.insert(vertex, vertices.len());
            edges.insert(
                Idx::try_from(vertices.len()).expect("Node# fits Idx"),
                Vec::new(),
            );
            vertices.push((vertex.to_string(), vertex_data.clone()));
        }

        for (vertex, _, vertex_edges) in data {
            let vertex = vertexname_to_idx.get(vertex).expect("Node exists");
            for (edge, edge_data) in vertex_edges {
                let edge = vertexname_to_idx
                    .get(edge)
                    .expect("edge destination exists");
                let edges = edges
                    .get_mut(&Idx::try_from(*vertex).expect("Idx fits"))
                    .expect("Node exists");
                edges.push((
                    Idx::try_from(*edge).expect("Fits"),
                    cost_func(&vertices[*vertex].1, edge_data),
                    edge_data.clone(),
                ));
            }
        }

        UniGraph { vertices, edges }
    }
}

impl<VT, ET, Idx, CT> BiGraph<VT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + std::fmt::Debug + Eq + std::hash::Hash,
    <Idx as TryFrom<usize>>::Error: std::fmt::Debug,
    VT: Clone,
    ET: Clone,
{
    /// Takes a unigraph and compresses it.
    ///
    /// Edges are made bidirectional before compression by copying edges.
    ///
    /// The should_combine function returns the new data and cost for two edges
    /// if the given vertex and its two edges are to be removed.
    pub fn compress<F>(unigraph: UniGraph<VT, ET, Idx, CT>, should_combine: F) -> Self
    where
        F: Fn(&VT, &VT, &VT, &ET, CT, &ET, CT) -> Option<(CT, ET)>,
        Idx: Into<usize> + Copy,
        CT: Copy,
    {
        // TODO: Split into From & compress as separate functions?
        // TODO: Add a new function?

        let mut graph = BiGraph {
            vertices: unigraph.vertices,
            edges: HashMap::new(),
        };

        // Add edges in both directions
        for (from_vertex, old_list) in unigraph.edges {
            for (vertex2, cost, edge_data) in &old_list {
                let list2 = graph.edges.entry(from_vertex).or_insert_with(Vec::new);
                if !list2.iter().any(|(vertex, _, _)| vertex == vertex2) {
                    list2.push((*vertex2, *cost, edge_data.clone()));
                }
                let list2 = graph.edges.entry(*vertex2).or_insert_with(Vec::new);
                if !list2.iter().any(|(vertex, _, _)| *vertex == from_vertex) {
                    list2.push((from_vertex, *cost, edge_data.clone()));
                }
            }
        }

        loop {
            let mut any_change = false;

            // HashMap<Idx, Vec<(Idx, CT, &'a ET)>>,
            let mut update = None;
            'find_edge: for (vertex1, edges) in &graph.edges {
                if edges.len() != 2 {
                    continue;
                }
                for (vertex2, cost, edge_data) in edges {
                    for (vertex3, cost3, edge_data3) in edges {
                        if vertex3 == vertex2 {
                            continue;
                        }
                        if let Some((cost, data)) = should_combine(
                            &graph.vertices[Idx::into(*vertex1)].1,
                            &graph.vertices[Idx::into(*vertex2)].1,
                            &graph.vertices[Idx::into(*vertex3)].1,
                            edge_data,
                            *cost,
                            edge_data3,
                            *cost3,
                        ) {
                            update = Some((*vertex1, *vertex2, *vertex3, (cost, data)));
                            any_change = true;
                            break 'find_edge;
                        }
                    }
                }
            }
            if let Some((vertex_to_remove, vertex1, vertex2, cost_and_data)) = update {
                graph.edges.remove(&vertex_to_remove);

                // Update edges at vertex1 from vertex_to_remove to vertex2
                let list = graph.edges.get_mut(&vertex1).expect("edge list");
                for (vertex, cost, data) in list.iter_mut() {
                    if *vertex == vertex_to_remove {
                        *vertex = vertex2;
                        *cost = cost_and_data.0;
                        *data = cost_and_data.1.clone();
                    }
                }
                // Update edges at vertex2 from vertex_to_remove to vertex1
                let list = graph.edges.get_mut(&vertex2).expect("edge list");
                for (vertex, cost, data) in list.iter_mut() {
                    if *vertex == vertex_to_remove {
                        *vertex = vertex1;
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

impl<VT, ET, Idx, CT> std::fmt::Debug for UniGraph<VT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + Into<usize> + std::fmt::Debug + Copy,
    VT: std::fmt::Debug + Clone,
    ET: std::fmt::Debug + Clone,
    CT: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "UniGraph {{")?;
        for (from_vertex, list) in &self.edges {
            for (vertex, cost, data) in list {
                writeln!(
                    fmt,
                    "  {} -({:?}, {:?})> {}",
                    self.vertices[(*from_vertex).into()].0,
                    cost,
                    data,
                    self.vertices[(*vertex).into()].0
                )?;
            }
        }
        writeln!(fmt, "}}")
    }
}

impl<VT, ET, Idx, CT> std::fmt::Debug for BiGraph<VT, ET, Idx, CT>
where
    Idx: TryFrom<usize> + Into<usize> + std::fmt::Debug + Copy,
    VT: std::fmt::Debug + Clone,
    ET: std::fmt::Debug + Clone,
    CT: std::fmt::Debug,
{
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "BiGraph {{")?;
        for (from_vertex, list) in &self.edges {
            for (vertex, cost, data) in list {
                writeln!(
                    fmt,
                    "  {} -({:?}, {:?})> {}",
                    self.vertices[(*from_vertex).into()].0,
                    cost,
                    data,
                    self.vertices[(*vertex).into()].0
                )?;
            }
        }
        writeln!(fmt, "}}")
    }
}
