use std::{collections::HashMap, fmt::Debug, fs::{File}, io::{self, BufRead, BufReader, Read}};
mod traversal;


#[derive(Debug)]
pub struct AdjacencyList {
    vertices : Vec<Vertex>,
    directed : bool,
    weighted : bool,
    vertex_label_idx : HashMap<String, usize>
}

impl AdjacencyList {
    pub fn load(file : File) -> AdjacencyList {
        let mut file_read = BufReader::new(file);
        
        let opt = parse_opt(&mut file_read).unwrap();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut file = String::new();
        file_read.read_to_string(&mut file).unwrap();

        let mut vertex_label_idx: HashMap<String, usize> = HashMap::new();

        for line in file.clone().lines() {
            let line : Vec<&str> = line
                .split(";")
                .nth(0)
                .unwrap()
                .split("-")
                .map(|s|s.trim_matches(&['<', '>', ' ']))
                .collect();

            assert!(line.len() == 2);

            for label in line {
                if !vertex_label_idx.contains_key(label) {
                    let curr_idx = vertices.len();
                    vertices.push(Vertex::new(curr_idx, label.to_string()));
                    vertex_label_idx.insert(label.to_string(), curr_idx);
                }
            }
        }

        for line in file.lines() {
            let line = line
                .split(";")
                .nth(0)
                .unwrap();

            // bidirected is <->, while normal (forward) is ->, no backward
            let bidirected = line.contains("<->");
            let line : Vec<&str> = line.split("-")
                .map(|s|s.trim_matches(&['<', '>', ' ']))
                .collect();

            assert!(line.len() == 2);
            assert!(vertex_label_idx.contains_key(line[1]));

            let (x, y) = (vertex_label_idx[line[0]], vertex_label_idx[line[1]]);
            if vertices[x].edges.iter().any(|e|e.target_idx == y){
                continue;
            }

            vertices[x].edges.push(Edge { target_idx : y, weight: 0 });
            if !opt.directed || bidirected {
                vertices[y].edges.push(Edge { target_idx : x, weight : 0});
            }
        };
        AdjacencyList { 
            vertices, 
            directed : opt.directed,
            weighted : opt.weighted,
            vertex_label_idx : vertex_label_idx
        }
    }
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

struct ParseOpt {
    directed : bool,
    weighted : bool,
}

fn parse_opt(file : &mut BufReader<File>) -> io::Result<ParseOpt> {
    let mut line =  String::new();
    file.read_line(& mut line)?;
    let opt : Vec<&str> = line
        .trim_matches(&['[', ']', '\n' , '\r'])
        .split(';')
        .map(|s| s.trim())
        .collect();
    
    assert!(opt.len() > 0);

    let directed = opt.iter().any(|s|*s == "DIRECTED");
    let weighted = opt.iter().any(|s|*s == "WEIGHTED");
    Ok(ParseOpt {directed, weighted})
}

#[derive(Clone, Debug)]
pub struct Edge {
    target_idx : usize,
    weight : i32,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    idx : usize,
    label  : String,
    pub edges : Vec<Edge>,
}

impl Vertex {
    pub fn new(idx :usize, label : String) -> Vertex {
        Vertex { idx, label, edges: Vec::new()}
    }
    pub fn degree(&self)->usize {
        self.edges.len()
    }
    pub fn idx(&self)->usize {
        self.idx
    }
    pub fn label(&self) -> &str {
        &self.label
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
    fn ne(&self, other: &Self) -> bool {
        self.idx != other.idx
    }
}