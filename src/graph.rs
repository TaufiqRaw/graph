use std::{cell::RefCell, clone, fmt::Debug, fs::{self, File}, io::{self, BufRead, BufReader}, rc::{self, Rc}, vec};
mod traversal;


type VertCell = RefCell<Vertex>;
#[derive(Debug)]
pub struct AdjacencyList {
    vertices : Vec<Rc<VertCell>>,
    directed : bool,
}

impl AdjacencyList {
    pub fn load(file : File) -> AdjacencyList {
        let mut file = BufReader::new(file);
        
        let opt = parse_opt(&mut file).unwrap();
        let mut vertices: Vec<Rc<VertCell>> = Vec::with_capacity(opt.n_vertex);
        for i in 0 .. opt.n_vertex {
            vertices.push(Rc::new(RefCell::new(Vertex::new(i+1))));
        } 

        for line in file.lines() {
            let line : Vec<usize> = line
                .unwrap()
                .split(" ")
                .map(|s| s.parse::<usize>().unwrap()-1)
                .collect();

            assert!(line.len() == 2);

            let (x, y) = (line[0], line[1]);
            if vertices[x].borrow().edges.iter().any(|v|v.upgrade().unwrap() == vertices[y]){
                continue;
            }
            vertices[x].borrow_mut().edges.push(Rc::downgrade(&vertices[y]));
            if !opt.directed {
                vertices[y].borrow_mut().edges.push(Rc::downgrade(&vertices[x]));
            }
        };
        AdjacencyList { 
            vertices, 
            directed : opt.directed,
        }
    }
    // pub fn count_ref(&self) {
    //     for el in &self.vertices {
    //         println!("{}", rc::Rc::strong_count(&el));
    //     };
    // }
    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

struct ParseOpt {
    n_vertex : usize,
    directed : bool
}

fn parse_opt(file : &mut BufReader<File>) -> io::Result<ParseOpt> {
    let mut line =  String::new();
    file.read_line(& mut line)?;
    let opt : Vec<&str> = line
        .trim_matches(&['[', ']', '\n' , '\r'])
        .split(';')
        .map(|s| s.trim())
        .collect();

    println!("{opt:?}");

    assert!(opt.len() > 0);

    let n_vertex = opt[0].parse::<usize>().unwrap();
    let directed = opt[1..].iter().any(|s|*s == "DIRECTED");
    Ok(ParseOpt {n_vertex, directed})
}

#[derive(Clone)]
pub struct Vertex {
    id : usize,
    pub edges : Vec<rc::Weak<VertCell>>,
}

impl Vertex {
    pub fn new(id :usize) -> Vertex {
        Vertex { id, edges: Vec::new() }
    }
    pub fn degree(&self)->usize {
        self.edges.len()
    }
    pub fn id(&self)->usize {
        self.id
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
    fn ne(&self, other: &Self) -> bool {
        self.id != other.id
    }
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let edges: Vec<usize> = self.edges.iter().map(|e| e.upgrade().unwrap().borrow().id).collect();
        let res = f.debug_struct(&format!("Vertex_{}", self.id))
            .field("vertices", &edges)
            .finish();
        res
    }
}