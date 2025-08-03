use std::{fs};

use crate::graph::AdjacencyList;

mod graph;

fn main() {
    let file = fs::File::open("test.graph").unwrap();
    let graph = AdjacencyList::load(file);

    let prim = graph.prim("Mulligan");
    for p in &prim {
        println!("({}) - parent :{}, distance :{}", p.label, match p.parent_idx {
            Some(p) => &prim[p].label,
            None => "None",
        }, p.distance);
    };
    
    // let mut n_edge = 0usize;
    // graph.bfs("Mulligan", |v|{
    //     print!("{}", v.data.label());
    //     let mut parent = v.parent();
    //     while parent.is_some() {
    //         let v = parent.unwrap();
    //         print!(" -> {}", v.data.label());
    //         parent = v.parent();
    //     } 
    //     println!("");
    // }, |_|{}, |_|{
    //     n_edge += 1;
    // });
    // println!("n edge = {}", n_edge);
}
