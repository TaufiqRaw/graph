use std::{cell::RefCell, fs};

use crate::graph::AdjacencyList;

mod graph;

fn main() {
    let file = fs::File::open("test.graph").unwrap();
    let graph = AdjacencyList::load(file);
    
    let mut n_edge = 0usize;
    let entry = RefCell::from(vec![0usize; graph.len()]);

    graph.dfs("Mulligan", |v, t_entry|{
        entry.borrow_mut()[v.idx()] = t_entry;
    }, |v, t_exit|{
        println!("{} has descentdant = {}", v.label(), (t_exit - entry.borrow()[v.idx()])/2);
    }, |_|{
        n_edge += 1;
    });
    println!("n edge = {}", n_edge);
    // graph.count_ref();
}
