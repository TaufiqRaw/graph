use std::rc;
use std::collections::VecDeque;

use crate::graph::{AdjacencyList, VertCell, Vertex};

#[derive(Clone, PartialEq)]
enum VertexState {
    Undiscoverd,
    Discovered,
    Processed,
} 

impl AdjacencyList {
    pub fn bfs(&self, id : usize, early_process : impl Fn(&rc::Rc<VertCell>), late_process : impl Fn(&rc::Rc<VertCell>)) {
        assert!(id<=self.len());
        let node_idx = id-1; //graph id start from 1;
        
        let mut states = vec![VertexState::Undiscoverd; self.len()];
        let mut parents: Vec<Option<usize>> = vec![None; self.len()];

        states[node_idx] = VertexState::Discovered;
        let mut queue = VecDeque::from(vec![self.vertices[node_idx].clone()]);
        while !queue.is_empty() {
            let vertex = queue.pop_front().unwrap();
            early_process(&vertex);
            let borrow_vert = vertex.borrow();
            for child in &borrow_vert.edges {
                let child = child.upgrade().unwrap();
                let child_idx = child.borrow().id-1;
                //TODO: process edge if graph directed
                if states[child_idx] == VertexState::Undiscoverd {
                    states[child_idx] = VertexState::Discovered;
                    parents[child_idx] = Some(borrow_vert.id - 1); 
                    queue.push_back(child);
                }
            }
            late_process(&vertex);
        }
    }
}