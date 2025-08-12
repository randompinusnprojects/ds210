use std::collections::{HashMap, HashSet};
use std::vec;

pub fn collect_paths_from_list(
    graph: &HashMap<String, HashSet<String>>,
    start_nodes: &Vec<String>,
    max_depth: usize,
) -> Vec<Vec<String>> {
    let mut all_paths = Vec::new();

    fn dfs(
        graph: &HashMap<String, HashSet<String>>,
        current: &String,
        max_depth: usize,
        path: &mut Vec<String>,
        all_paths: &mut Vec<Vec<String>>,
    ) {
        if path.len() > max_depth {
            return;
        }

        path.push(current.clone());

        if let Some(neighbors) = graph.get(current) {
            for neighbor in neighbors {
                if !path.contains(neighbor) {
                    dfs(graph, neighbor, max_depth, path, all_paths);
                }
            }
        }

        all_paths.push(path.clone());
        path.pop();
    }

    for start in start_nodes {
        let mut path = Vec::new();
        dfs(graph, start, max_depth, &mut path, &mut all_paths);
    }

    all_paths
}


pub fn calculate_reuse_score(paths: &Vec<Vec<String>>) -> HashMap<String, usize> {
    let mut count: HashMap<String, usize> = HashMap::new();

    for path in paths {
        if path.len() <= 2 {
            continue; // 중간 노드가 없음
        }

        for node in &path[1..path.len()-1] { // 중간 노드만
            *count.entry(node.clone()).or_insert(0) += 1;
        }
    }

    count
}