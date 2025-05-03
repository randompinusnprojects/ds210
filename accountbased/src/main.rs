use plotters::prelude::*;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::vec;

mod fileread;
mod cycles;
mod intermed;


fn cycle_search(
    start: &String,
    graph: &HashMap<String, HashSet<String>>,
    max_depth: usize,
) -> Option<Vec<String>> {
    let mut path = Vec::new();
    let mut cycle = None;

    dfs_cycle_check(graph, start, start, max_depth, &mut path, &mut cycle);

    cycle
}

fn dfs_cycle_check(
    graph: &HashMap<String, HashSet<String>>,
    current: &String,
    start: &String,
    max_depth: usize,
    path: &mut Vec<String>,
    found_cycle: &mut Option<Vec<String>>,
) {
    if path.len() > max_depth + 1 || found_cycle.is_some() {
        return;
    }

    path.push(current.clone());

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            if neighbor == start && path.len() >= 2 {
                *found_cycle = Some(path.clone());
                break;
            } else {
                dfs_cycle_check(graph, neighbor, start, max_depth, path, found_cycle);
            }
        }
    }

    path.pop();
}



fn main() {
    println!("Reading.");
    let labels = fileread::read_to_hashmap("../../elliptic_txs_classes.csv");
    println!("Reading..");
    let edges = fileread::read_file_directed("../../elliptic_txs_edgelist.csv");
    println!("Reading...");
    let timestamp = fileread::read_file_directed("../../elliptic_txs_features.csv");

    // println!("{:?}", features);

    let mut licit_nodes = Vec::new();
    
    for (k, v) in labels.iter() {
        if v == "1" {
            licit_nodes.push(k.to_string());
        }
    }

    let mut illicit_nodes = Vec::new();

    for (k, v) in labels.iter() {
        if v == "2" {
            illicit_nodes.push(k.to_string());
        }
    }

    println!("I found illicit nodes!");

    for node in &illicit_nodes {
        // println!("I'm here");
        if let Some(cycle) = cycle_search(node, &edges, 7) {
            //println!("Heya");
            println!("Cycle found from {}: {:?}", node, cycle);
        }
    }

    let mut degree_list: HashMap<String, usize> = HashMap::new();

    for (k, v) in edges.iter() {
        degree_list.insert(k.clone(), v.len());
    }

    let sum: usize = degree_list.values().sum();
    let avg_degree = sum as f64 / degree_list.len() as f64;

    println!("Average out-degree: {:.2}", avg_degree);


    println!("number of licit nodes: {}", licit_nodes.len());
    println!("number of illciit nodes: {}", illicit_nodes.len());

    for k in (0..10) {
        println!("k is {}", k);
        let licit_paths = intermed::collect_paths_from_list(&edges, &licit_nodes, k);
        let illicit_paths = intermed::collect_paths_from_list(&edges, &illicit_nodes, k);
        println!("number of licit paths: {}", licit_paths.len());
        println!("number of illicit paths: {}", illicit_paths.len());

        let reuse_scores_licit = intermed::calculate_reuse_score(&licit_paths);
        let reuse_scores_illicit = intermed::calculate_reuse_score(&illicit_paths);
        let mut sorted_max_inter = reuse_scores_licit.iter().collect::<Vec<_>>();
        sorted_max_inter.sort_by(|a, b| b.1.cmp(a.1));
        let sorted_max_inter_licit = sorted_max_inter.into_iter().take(10).collect::<Vec<_>>();

        sorted_max_inter = reuse_scores_illicit.iter().collect::<Vec<_>>();
        sorted_max_inter.sort_by(|a, b| b.1.cmp(a.1));
        let sorted_max_inter_illicit = sorted_max_inter.into_iter().take(10).collect::<Vec<_>>();

        println!("licit intermeds: {:?}", sorted_max_inter_licit);
        println!("illicit intermeds: {:?}", sorted_max_inter_illicit);
    }
    let licit_paths = intermed::collect_paths_from_list(&edges, &licit_nodes, 7);
    let illicit_paths = intermed::collect_paths_from_list(&edges, &illicit_nodes, 7);
    println!("number of licit paths: {}", licit_paths.len());
    println!("number of illicit paths: {}", illicit_paths.len());

    let reuse_scores_licit = intermed::calculate_reuse_score(&licit_paths);
    let reuse_scores_illicit = intermed::calculate_reuse_score(&illicit_paths);
    let mut sorted_max_inter = reuse_scores_licit.iter().collect::<Vec<_>>();
    sorted_max_inter.sort_by(|a, b| b.1.cmp(a.1));
    let sorted_max_inter_licit = sorted_max_inter.into_iter().take(10).collect::<Vec<_>>();

    sorted_max_inter = reuse_scores_illicit.iter().collect::<Vec<_>>();
    sorted_max_inter.sort_by(|a, b| b.1.cmp(a.1));
    let sorted_max_inter_illicit = sorted_max_inter.into_iter().take(10).collect::<Vec<_>>();

    println!("licit intermeds: {:?}", sorted_max_inter_licit);
    println!("illicit intermeds: {:?}", sorted_max_inter_illicit);

    let illicit_path_inter_nodes: Vec<String> = sorted_max_inter_illicit.iter().map(|(k, v)| k.to_string()).collect();
    for node in &illicit_path_inter_nodes {
        if illicit_nodes.contains(&node) {
            println!("{} is illciit", node);
        }
        else if licit_nodes.contains(&node) {
            println!("{} is licit", node);
        }
        else {
            println!("{} is unknown", node);
        }
    }
    let illicit_path_inter_paths = intermed::collect_paths_from_list(&edges, &illicit_path_inter_nodes, 7);

}
