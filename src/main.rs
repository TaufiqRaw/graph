use std::fs;

use crate::graph::AdjacencyList;

mod graph;

fn main() {
    let file = fs::File::open("test.graph").unwrap();
    let graph = AdjacencyList::load(file);
    graph.bfs(1, |vert|{
        println!("vertex #{} ", vert.borrow().id())
    }, |_| {});
    // graph.count_ref();
}
