use std::collections::{HashMap, HashSet};

fn dfs_loose_cycle(
    graph: &HashMap<String, HashSet<String>>,
    current: &String,
    start: &String,
    k: usize,
    path: &mut Vec<String>,
    cycles: &mut Vec<Vec<String>>,
) {
    if path.len() > k + 1 {
        return;
    }

    path.push(current.clone());

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            // println!("Don't worry, I'm doing sth");
            if neighbor == start && path.len() > 2 {
                cycles.push(path.clone());
            } else {
                dfs_loose_cycle(graph, neighbor, start, k, path, cycles);
            }
        }
    }
    // println!("Don't worry, I'm doing sth");

    path.pop();
}



pub fn find_k_cycles(
    graph: &HashMap<String, HashSet<String>>, 
    k: usize
) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();

    for start in graph.keys() {
        // println!("Yoohoo!");
        let mut path = Vec::new();
        dfs_loose_cycle(graph, start, start, k, &mut path, &mut cycles);
    }
    println!("Yoohoo!");

    cycles
}
