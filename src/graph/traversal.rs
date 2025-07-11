use std::rc::{self, Rc};
use std::{collections::VecDeque, rc::Weak};

use crate::graph::{AdjacencyList, Edge, VertCell, Vertex};

#[derive(Clone, PartialEq)]
enum VertexState {
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
        let mut parents: Vec<Weak<VertCell>> = vec![Weak::new(); self.len()];

        states[vertex_idx] = VertexState::Discovered;
        let mut queue = VecDeque::from(vec![self.vertices[vertex_idx].clone()]);
        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            early_vertex(&vertex.borrow());
            let borrow_vert = vertex.borrow();
            for edge in &borrow_vert.edges {
                let child = edge.vertex.upgrade().unwrap();
                let child_idx = child.borrow().idx;

                process_edge(edge);
                if states[child_idx] == VertexState::Undiscoverd {
                    states[child_idx] = VertexState::Discovered;
                    parents[child_idx] = Rc::downgrade(&vertex);
                    queue.push_back(child);
                }
            }
            late_vertex(&vertex.borrow());
            states[borrow_vert.idx] = VertexState::Processed;
        }
    }

    pub fn dfs(
        &self,
        label: &str,
        mut early_vertex: impl FnMut(&Vertex, usize),
        mut late_vertex: impl FnMut(&Vertex, usize),
        mut process_edge: impl FnMut(&Edge),
    ) {
        assert!(self.vertex_label_idx.contains_key(label));

        let vertex_idx = self.vertex_label_idx[label];
        let mut states: Vec<VertexState> = vec![VertexState::Undiscoverd; self.len()];
        let mut parents: Vec<Weak<VertCell>> = vec![Weak::new(); self.len()];
        let mut time: usize = 0;

        self.do_dfs(
            &self.vertices[vertex_idx],
            &mut states,
            &mut parents,
            &mut time,
            &mut early_vertex,
            &mut late_vertex,
            &mut process_edge,
        );
    }

    fn do_dfs(
        &self,
        vertex: &Rc<VertCell>,
        states: &mut Vec<VertexState>,
        parents: &mut Vec<Weak<VertCell>>,
        time: &mut usize,
        early_vertex: &mut impl FnMut(&Vertex, usize),
        late_vertex: &mut impl FnMut(&Vertex, usize),
        process_edge: &mut impl FnMut(&Edge),
    ) {
        states[vertex.borrow().idx()] = VertexState::Discovered;
        early_vertex(&vertex.borrow(), time.clone());
        *time += 1;
        for edge in &vertex.borrow().edges {
            process_edge(&edge);
            let next_vertex = edge.vertex.upgrade().unwrap();
            if states[next_vertex.borrow().idx()] == VertexState::Undiscoverd {
                parents[next_vertex.borrow().idx()] = Rc::downgrade(vertex);
                self.do_dfs(
                    &next_vertex,
                    states,
                    parents,
                    time,
                    early_vertex,
                    late_vertex,
                    process_edge,
                );
            }
        }
        late_vertex(&vertex.borrow(), time.clone());
        states[vertex.borrow().idx] = VertexState::Processed;
        *time += 1;
    }
}
