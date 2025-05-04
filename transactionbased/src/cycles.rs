use std::collections::{HashMap, HashSet};

/// Performs a depth-first search to detect "loose" cycles of up to length `k + 1`.
///
/// A *loose* cycle here refers to a cycle that:
/// - Starts and ends at the same node (`start`)
/// - Has length at most `k + 1`
/// - May contain repeated nodes (except immediate revisits are avoided by construction)
///
/// # Arguments
/// * `graph` - Directed graph represented as an adjacency list.
/// * `current` - Node currently being visited.
/// * `start` - Original node where cycle search began.
/// * `k` - Maximum cycle length (not including the repeated start node).
/// * `path` - Current path being built.
/// * `cycles` - Collected list of valid cycles.
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
    // println!("Don't worry, I'm doing sth"); // for those of you that doubt if this is working

    path.pop();
}


/// Initiates DFS from each node to detect cycles of length up to `k + 1`.
///
/// # Arguments
/// * `graph` - The input graph as a directed adjacency list.
/// * `k` - Max cycle length (excluding start node repetition).
///
/// # Returns
/// A vector of all found cycles represented as vectors of node IDs.
pub fn find_k_cycles(
    graph: &HashMap<String, HashSet<String>>, 
    k: usize
) -> Vec<Vec<String>> {
    println!("Started finding cycles!");
    let mut cycles = Vec::new();

    for start in graph.keys() {
        // println!("Yoohoo!");
        let mut path = Vec::new();
        dfs_loose_cycle(graph, start, start, k, &mut path, &mut cycles);
    }

    cycles
}
