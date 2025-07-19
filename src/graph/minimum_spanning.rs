use crate::graph::{AdjacencyList};

macro_rules! run {
    ($x:block until $y:expr) => {{
        while {
            $x;
            !$y
            } {}
    }};
}

pub struct SpanningTree {
    pub label : String,
    pub parent_idx : Option<usize>,
    pub distance : usize,
}

#[derive(Clone, Copy)]
enum VertState{
    Undiscovered,
    NotInTree(usize),
    InTree(usize)
}

impl VertState { 
    pub fn get_or_insert(&mut self,mut x:impl FnMut()->usize)-> &mut usize {
        match self {
            VertState::Undiscovered => {
                *self = Self::NotInTree(x());
                if let Self::NotInTree(x) = self{
                    x
                }else {
                    panic!("imposible");
                }
            },
            VertState::InTree(x)=>{x},
            VertState::NotInTree(x)=>{x},
        }
    }
    pub fn is_in_tree(&self) -> bool{
        match self {
            VertState::Undiscovered => false,
            VertState::NotInTree(_)=>false,
            VertState::InTree(_)=>true,
        }
    }
    pub fn unwrap(&self) -> usize {
        match self {
            VertState::InTree(x) => {*x},
            VertState::NotInTree(x) => {*x},
            VertState::Undiscovered => panic!("cant unwrap"),
        }
    }
}

impl AdjacencyList {
    pub fn prim(&self, start_key : &str)-> Vec<SpanningTree> {
        assert!(self.nonnegative);
        assert!(!self.directed);
        let start_idx = self.vertex_label_idx.get(start_key).unwrap().clone();
        
        let mut distances : Vec<VertState> = vec![VertState::Undiscovered;self.len()];
        let mut parent_idxs : Vec<Option<usize>> = vec![None; self.len()];
        
        distances[start_idx] = VertState::InTree(0);
        let mut cur_idx = start_idx;
        run!({
            distances[cur_idx] = VertState::InTree(distances[cur_idx].unwrap());
            for edge in &self.vertices[cur_idx].edges {
                if parent_idxs[cur_idx].is_some_and(|idx| idx == edge.target_idx){
                    continue;
                }
                let old_distance = distances[edge.target_idx].get_or_insert(||{
                    parent_idxs[edge.target_idx] = Some(cur_idx);
                    edge.weight as usize
                });
                if *old_distance > edge.weight as usize {
                    parent_idxs[edge.target_idx] = Some(cur_idx);
                    *old_distance = edge.weight as usize;
                }
            }
            //select smallest distance mf
            let mut smallest_w = usize::MAX; 
            for candidate_idx in 0..self.len() {
                if let VertState::NotInTree(candidate_w) = distances[candidate_idx] {
                    if candidate_w < smallest_w {
                        smallest_w = candidate_w;
                        cur_idx = candidate_idx;
                    }
                }
            };
        } until distances[cur_idx].is_in_tree());
        let mut result_tree = Vec::with_capacity(self.len());
        for i in 0..self.len(){
            result_tree.push(SpanningTree { 
                label: self.vertices[i].label().to_owned(),
                parent_idx: parent_idxs[i], 
                distance: distances[i].unwrap() });
        };
        result_tree
    }
}