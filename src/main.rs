use std::{fs};

use crate::graph::AdjacencyList;

mod graph;

fn main() {
    let file = fs::File::open("test.graph").unwrap();
    let graph = AdjacencyList::load(file);
    
    let mut n_edge = 0usize;
    graph.dfs("Mulligan", |v|{
        print!("{}", v.data.label());
        let mut parent = v.parent();
        while parent.is_some() {
            let v = parent.unwrap();
            print!(" -> {}", v.data.label());
            parent = v.parent();
        } 
        println!("");
    }, |_|{}, |_|{
        n_edge += 1;
    });
    println!("n edge = {}", n_edge);
    // graph.count_ref();
}
