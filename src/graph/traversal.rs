use std::{collections::VecDeque};

use crate::graph::{AdjacencyList, Edge, Vertex};

#[derive(Clone, PartialEq)]
pub enum VertexState {
    Undiscoverd,
    Discovered,
    Processed,
}

impl AdjacencyList {
    pub fn bfs(
        &self,
        label: &str,
        mut early_vertex: impl FnMut(&Vertex),
        mut late_vertex: impl FnMut(&Vertex),
        mut process_edge: impl FnMut(&Edge),
    ) {
        assert!(self.vertex_label_idx.contains_key(label));
        let vertex_idx = self.vertex_label_idx[label];

        let mut states = vec![VertexState::Undiscoverd; self.len()];
        let mut parents: Vec<Option<usize>> = vec![None; self.len()];

        states[vertex_idx] = VertexState::Discovered;
        let mut queue = VecDeque::from(vec![&self.vertices[vertex_idx]]);
        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            early_vertex(&vertex);
            for edge in &vertex.edges {
                let child = &self.vertices[edge.target_idx];

                process_edge(edge);
                if states[child.idx()] == VertexState::Undiscoverd {
                    states[child.idx()] = VertexState::Discovered;
                    parents[child.idx()] = Some(vertex.idx());
                    queue.push_back(child);
                }
            }
            late_vertex(&vertex);
            states[vertex.idx()] = VertexState::Processed;
        }
    }

    pub fn dfs(
        &self,
        label: &str,
        mut early_vertex: impl FnMut(DfsVert),
        mut late_vertex: impl FnMut(DfsVert),
        mut process_edge: impl FnMut(&Edge),
    ) {
        assert!(self.vertex_label_idx.contains_key(label));

        let vertex_idx = self.vertex_label_idx[label];
        let mut states: Vec<VertexState> = vec![VertexState::Undiscoverd; self.len()];
        let mut parents: Vec<Option<usize>> = vec![None; self.len()];

        self.do_dfs(
            &self.vertices[vertex_idx],
            &mut states,
            &mut parents,
            &mut early_vertex,
            &mut late_vertex,
            &mut process_edge,
        );
    }

    fn do_dfs(
        &self,
        vertex: &Vertex,
        states: &mut Vec<VertexState>,
        parents: &mut Vec<Option<usize>>,
        early_vertex: &mut impl FnMut(DfsVert),
        late_vertex: &mut impl FnMut(DfsVert),
        process_edge: &mut impl FnMut(&Edge),
    ) {
        states[vertex.idx()] = VertexState::Discovered;
        early_vertex(DfsVert { data: vertex, g: self, parents: &parents, states: &states });
        for edge in &vertex.edges {
            process_edge(&edge);
            let next_vertex = &self.vertices[edge.target_idx];
            if states[next_vertex.idx()] == VertexState::Undiscoverd {
                parents[next_vertex.idx()] = Some(vertex.idx());
                self.do_dfs(
                    &next_vertex,
                    states,
                    parents,
                    early_vertex,
                    late_vertex,
                    process_edge,
                );
            }
        }
        late_vertex(DfsVert { data: vertex, g: self, parents: &parents, states: &states});
        states[vertex.idx] = VertexState::Processed;
    }
}

pub struct DfsVert<'a> {
    pub data : &'a Vertex,

    g : &'a AdjacencyList,
    parents : &'a Vec<Option<usize>>,
    states : &'a Vec<VertexState>,
}

impl<'a> DfsVert<'a> {
    pub fn parent(&self) -> Option<DfsVert<'a>> {
        let parent_idx = self.parents[self.data.idx]?;
        let parent_vert = &self.g.vertices[parent_idx];
        Some(
            DfsVert { data: parent_vert, g: self.g, parents: self.parents, states: self.states}
        )
    }
    pub fn state(&self) -> &VertexState {
        &self.states[self.data.idx]
    }
}