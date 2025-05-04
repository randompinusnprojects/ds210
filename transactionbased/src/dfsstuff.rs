use plotters::prelude::*;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;

/// Performs timestamp-filtered DFS to collect all reachable nodes from start nodes.
/// 
/// Skips revisiting nodes and enforces monotonic time increase.
/// 
/// # Arguments
/// * `graph`, `timestamps` - The transaction graph and its timestamps.
/// * `current` - The node currently being visited.
/// * `depth` - Current recursion depth.
/// * `reachable` - Accumulates all reachable nodes.
/// * `max_depth` - Max search depth to avoid combinatorial explosion.
pub fn dfs_collect_reachable(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    current: &String,
    depth: usize,
    visited_on_path: &mut HashSet<String>,
    reachable: &mut HashSet<String>,
    max_depth: usize,
) {
    if depth >= max_depth || visited_on_path.contains(current) {
        return;
    }

    visited_on_path.insert(current.clone());
    reachable.insert(current.clone());

    if let Some(neighbors) = graph.get(current) {
        let current_ts = timestamps.get(current).copied().unwrap_or(0);
        for neighbor in neighbors {
            let neighbor_ts = timestamps.get(neighbor).copied().unwrap_or(usize::MAX);
            if neighbor_ts >= current_ts {
                dfs_collect_reachable(
                    graph,
                    timestamps,
                    neighbor,
                    depth + 1,
                    visited_on_path,
                    reachable,
                    max_depth,
                );
            }
        }
    }

    visited_on_path.remove(current);
}

/// DFS to collect full valid paths from `start → target` while respecting timestamp ordering.
/// 
/// # Returns
/// Fills `all_paths` with paths satisfying the constraints.
///
pub fn dfs_collect_paths(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    current: &String,
    target: &String,
    path: &mut Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
    visited: &mut HashSet<String>,
    depth: usize,
    max_depth: usize,
) {
    if depth > max_depth || visited.contains(current) {
        return;
    }

    path.push(current.clone());
    visited.insert(current.clone());

    if current == target && depth > 1 {
        all_paths.push(path.clone());
    } else if let Some(neighbors) = graph.get(current) {
        let current_ts = timestamps.get(current).copied().unwrap_or(0);
        for neighbor in neighbors {
            let neighbor_ts = timestamps.get(neighbor).copied().unwrap_or(usize::MAX);
            if neighbor_ts >= current_ts {
                // println!("how long are you? : {}", all_paths.len());
                dfs_collect_paths(
                    graph, timestamps, neighbor, target,
                    path, all_paths, visited, depth + 1, max_depth
                );
            }
        }
    }

    path.pop();
    visited.remove(current);
}

/// Summary DFS: Instead of storing all paths, just records number of valid paths and their cumulative depth.
/// 
/// Enforces max path count per (start, target) to avoid explosion.
/// 
/// # Updates
/// * `stats`: (start, target) → (num_paths, total_depth)
pub fn dfs_summary(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    current: &String,
    target: &String,  
    depth: usize,
    visited_on_path: &mut HashSet<String>,
    stats: &mut HashMap<(String, String), (usize, usize)>,
    start: &String,
    max_depth: usize,
    max_path: usize,
) {
    if depth >= max_depth {
        // println!("Too deep");
        return;
    }    
    
    if visited_on_path.contains(current) {
        return;
    }

    if current == target && depth > 1 {
        let entry = stats.entry((start.clone(), target.clone())).or_insert((0, 0));
        if entry.0 >= max_path {
            return;
        }
        entry.0 += 1;
        entry.1 += depth;
        // println!("Counting... {:?}", entry);
        return; 
    }

    visited_on_path.insert(current.clone());

    if let Some(neighbors) = graph.get(current) {
        let current_ts = timestamps.get(current).copied().unwrap_or(0);
        for neighbor in neighbors {
            let neighbor_ts = timestamps.get(neighbor).copied().unwrap_or(usize::MAX);
            // println!("how long are you? : {}", depth);
            if neighbor_ts >= current_ts && depth < max_depth {
                // println!(
                    // "Depth {}: {} → {} | cur_ts: {} neigh_ts: {} | visited: {:?}",
                    // depth,
                    // current,
                    // neighbor,
                    // current_ts,
                    // neighbor_ts,
                    // visited_on_path
                // );
                
                dfs_summary(
                    graph,
                    timestamps,
                    neighbor,
                    target,
                    depth + 1,
                    visited_on_path,
                    stats,
                    start,
                    max_depth,
                    max_path,
                );
            }
        }
    }

    visited_on_path.remove(current);
}

/// Entry point for DFS summary given multiple start/end node combinations.
/// 
/// # Returns
/// Map from (start, end) → (num paths, total depth).
pub fn summarize_paths_to_targets(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    start_nodes: &Vec<String>,
    end_nodes: &Vec<String>,
    max_depth: usize,
    max_path: usize,
) -> HashMap<(String, String), (usize, usize)> {
    let mut stats = HashMap::new();
    let target_set: HashSet<String> = end_nodes.iter().cloned().collect();

    for start in start_nodes {
        for target in end_nodes {
            let mut visited = HashSet::new();
            dfs_summary(graph, timestamps, start, target, 1, &mut visited, &mut stats, start, max_depth, max_path);
        }
    }
    
    for ((start, end), (count, total_depth)) in &stats {
        let avg_depth = *total_depth as f64 / *count as f64;
        println!("{} → {}: {} paths, avg depth {:.2}", start, end, count, avg_depth);
    }
    stats
}