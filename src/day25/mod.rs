use std::collections::{HashSet, HashMap};

#[derive(Clone)]
struct Graph {
    vertex_count: usize,
    edge_count: usize,
    edges: Vec<(String, String)>,
    vertices: HashMap<String, Vertex>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vertex {
    name: String,
    parent_vertex: String,
    rank: u32,
}

pub fn run_day() {
    let input_str = include_str!("input.txt");
    let graph = parse_input(input_str);
    println!("Day 25 Part 1: Result of multiplying subset sizes is {}", part_one(&graph));
}

fn parse_input(input_str: &str) -> Graph {
    let mut edges = Vec::new();
    let mut known_vertices = HashSet::new();

    for line in input_str.lines() {
        let (src, dest_str) = line.split_once(": ").unwrap();
        if !known_vertices.contains(src) {
            known_vertices.insert(src.to_string());
        }

        for dest in dest_str.split(" ") {
            if !known_vertices.contains(dest) {
                known_vertices.insert(dest.to_string());
            }

            edges.push((src.to_string(), dest.to_string()));
        }
    }

    let vertices: HashMap<String, Vertex> = known_vertices
        .iter()
        .map(|name| {
            let vertex = Vertex {
            name: name.to_string(),
            parent_vertex: name.to_string(),
            rank: 0,
            };
            (vertex.name.clone(), vertex)
        })
        .collect();

    Graph {
        vertex_count: known_vertices.len(),
        edge_count: edges.len(),
        edges,
        vertices,
    }
}

fn part_one(graph: &Graph) -> usize {
    let mut subset_a_size = 0;
    let mut subset_b_size = 0;

    loop {
        // do Karger's algorithm to find minimum cut
        // (this is probabilistic, so we may need to keep trying to
        //  find the best one, which is guaranteed by the problem to be 3)

        let mut contracted_vertices = graph.vertices.clone();
        let mut vertices = graph.vertex_count;

        // contract graph until it only has two vertices
        while vertices > 2 {
            let random_edge_idx = (rand::random::<f64>() * graph.edge_count as f64) as usize;
            let random_edge = &graph.edges[random_edge_idx];

            let source_parent = find_parent_vertex(&contracted_vertices, &random_edge.0);
            let dest_parent = find_parent_vertex(&contracted_vertices, &random_edge.1);

            // if source and dest are already under the same node, this edge doesn't need to be contracted
            if source_parent == dest_parent {
                continue;
            }

            // contract the edge
            contract(&mut contracted_vertices, &source_parent, &dest_parent);
            vertices -= 1;
        }

        // now determine how many edges are left between the two super vertices
        let mut min_cut = 0;
        let mut parent_a = "".to_string();
        let mut parent_b = "".to_string();
        for (source, dest) in graph.edges.iter() {
            let source_parent = find_parent_vertex(&contracted_vertices, source);
            let dest_parent = find_parent_vertex(&contracted_vertices, dest);
            if source_parent != dest_parent {
                min_cut += 1;
                parent_a = source_parent.clone();
                parent_b = dest_parent.clone();
            }
        }

        // if it's 3, we're done, gotta count the sizes of the subsets
        if min_cut == 3 {
            for (vertex, _) in graph.vertices.iter() {
                let parent_vertex = find_parent_vertex(&contracted_vertices, vertex);
                if parent_vertex == parent_a {
                    subset_a_size += 1;
                } else if parent_vertex == parent_b {
                    subset_b_size += 1;
                } else {
                    panic!("Didn't contract graph correctly! {parent_a} or {parent_b} aren't {parent_vertex}");
                }
            }
            break;
        }
    }

    subset_a_size * subset_b_size
}

fn find_parent_vertex(vertices: &HashMap<String, Vertex>, name: &str) -> String {
    let mut parent_vertex = vertices[name].parent_vertex.to_string();

    // if this vertex has a parent, follow it up the tree to find the root
    if parent_vertex != name {
        parent_vertex = find_parent_vertex(vertices, &parent_vertex).to_string();
    }

    parent_vertex
}

fn contract(vertices: &mut HashMap<String, Vertex>, source: &str, dest: &str) {
    // attach the smaller rank tree under the higher rank tree
    if vertices[source].rank < vertices[dest].rank {
        vertices.get_mut(source).unwrap().parent_vertex = dest.to_string();
    } else if vertices[source].rank > vertices[dest].rank {
        vertices.get_mut(dest).unwrap().parent_vertex = source.to_string();
    } else {
        //if both are the same, put one under the other and increment rank of the higher one
        vertices.get_mut(source).unwrap().parent_vertex = dest.to_string();
        vertices.get_mut(dest).unwrap().rank += 1;
    }
}
