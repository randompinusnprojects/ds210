use plotters::prelude::*;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;

fn read_to_hashmap(path: &str) -> HashMap<String, String> {
    let mut result: HashMap<String, String> = HashMap::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(',').collect();
        let k = v[0].to_string();
        let y = v[1].to_string();

        result.insert(k, y);
    }

    return result
}

fn read_file_directed(path: &str) -> HashMap<String, HashSet<String>> {
    let mut result: HashMap<String, HashSet<String>> = HashMap::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    let mut line_number = 0;
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        // println!("{}", line_str);
        let v: Vec<&str> = line_str.trim().split(',').collect();
        line_number += 1;
        if line_number == 1 {
            continue;
        }
        let k = v[0].to_string();
        let y = v[1].to_string();
        // println!("{}, {}", k, y);
        
        if let Some(nodes) = result.get(&k) { // if k exists in hashmap
            // insert y into the hashset
            let mut nodes = nodes.clone();
            nodes.insert(y);
            result.insert(k, nodes);
        }
        else { // if k doesn't exist in hashmap
            // create a new hashset and insert y into it
            let mut first: HashSet<String> = HashSet::new();
            first.insert(y);
            result.insert(k, first);
        }
        
    }
    return result
}

fn dfs_with_time_pruned(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    current: &String,
    path: &mut Vec<String>,
    all_paths: &mut Vec<Vec<String>>,
) {
    path.push(current.clone());
    let current_ts = timestamps.get(current).copied().unwrap_or(0);

    let mut extended = false;

    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            let neighbor_ts = timestamps.get(neighbor).copied().unwrap_or(usize::MAX);

            if !path.contains(neighbor) && neighbor_ts >= current_ts {
                extended = true;
                dfs_with_time_pruned(graph, timestamps, neighbor, path, all_paths);
            }
        }
    }

    // 저장 조건: 더 이상 확장 불가한 종단 경로만 저장
    if !extended {
        all_paths.push(path.clone());
    }

    path.pop();
}

fn collect_maximal_paths_time_filtered(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    start_nodes: &Vec<String>,
) -> Vec<Vec<String>> {
    let mut all_paths = Vec::new();

    for start in start_nodes {
        let mut path = Vec::new();
        dfs_with_time_pruned(graph, timestamps, start, &mut path, &mut all_paths);
    }

    all_paths
}

#[derive(Debug, Clone)]
struct TxEdge {
    txname: String,
    start: usize,      // 계좌 A
    end: usize,        // 계좌 B
    timestamp: usize,  // features에서 추출한 시점
}

fn assign_accounts_from_paths(
    paths: &Vec<Vec<String>>,
    timestamps: &HashMap<String, usize>,
) -> HashMap<String, TxEdge> {
    let mut tx_to_account: HashMap<String, TxEdge> = HashMap::new();
    let mut next_account_id = 0;

    for path in paths {
        let mut prev_end: Option<usize> = None;
        for tx in path {
            if tx_to_account.contains_key(tx) {
                prev_end = Some(tx_to_account[tx].end); // 이미 있는 경우 skip
                continue;
            }

            let ts = *timestamps.get(tx).unwrap_or(&0);
            let start = prev_end.unwrap_or_else(|| {
                let id = next_account_id;
                next_account_id += 1;
                id
            });
            let end = {
                let id = next_account_id;
                next_account_id += 1;
                id
            };

            let edge = TxEdge {
                txname: tx.clone(),
                start,
                end,
                timestamp: ts,
            };

            tx_to_account.insert(tx.clone(), edge);
            prev_end = Some(end);
        }
    }

    tx_to_account
}

fn build_weighted_account_graph(
    tx_edges: &HashMap<String, TxEdge>
) -> HashMap<usize, Vec<(usize, usize)>> {
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    for edge in tx_edges.values() {
        let entry = graph.entry(edge.start).or_insert_with(Vec::new);

        if let Some(pos) = entry.iter_mut().find(|(to, _)| *to == edge.end) {
            pos.1 += 1; // 이미 있으면 카운트 증가
        } else {
            entry.push((edge.end, 1)); // 새 연결 추가
        }
    }

    graph
}

fn main() {
    println!("Reading.");
    let labels = read_to_hashmap("../../elliptic_txs_classes.csv");
    println!("Reading..");
    let edges = read_file_directed("../../elliptic_txs_edgelist.csv");
    println!("Reading...");
    let raw_timestamps = read_to_hashmap("../../elliptic_txs_features.csv");

    let timestamps: HashMap<String, usize> = raw_timestamps
        .into_iter()
        .filter_map(|(k, v)| v.parse::<usize>().ok().map(|ts| (k, ts)))
        .collect();

    println!("Finished reading!");

    let mut illicit_nodes = Vec::new();
    for (k, v) in labels.iter() {
        if v == "2" {
            illicit_nodes.push(k.to_string());
        }
    }

    // let nodes = labels.keys().cloned().collect::<Vec<String>>();
    let batch_size = 100;
    let mut all_paths = Vec::new();

    for i in (0..illicit_nodes.len()).step_by(batch_size) {
        println!("Ayo let's see where my limit is {}", i);
        let end = usize::min(i + batch_size, illicit_nodes.len());
        let batch = &illicit_nodes[i..end].to_vec();
        let paths = collect_maximal_paths_time_filtered(&edges, &timestamps, batch);
        all_paths.extend(paths);
    }


    println!("Length of all_paths {}", all_paths.len());
    println!("{:?}", all_paths.get(0));

    let accounts_from_paths = assign_accounts_from_paths(&all_paths, &timestamps);
    for (tx, edge) in accounts_from_paths.iter() {
        // println!("{}: {:?} -> {:?}", tx, edge.start, edge.end);
    }

    let acc_graph = build_weighted_account_graph(&accounts_from_paths);

    for (from, tos) in acc_graph.iter() {
        println!("From account {}:", from);
        for (to, count) in tos {
            println!("  → to {} ({} times)", to, count);
        }
    }

    let mut freq_histogram: HashMap<usize, usize> = HashMap::new();
    for edges in acc_graph.values() {
        for (_to, count) in edges {
            *freq_histogram.entry(*count).or_insert(0) += 1;
        }
    }

    println!("Edge weight distribution:");
    for (count, freq) in freq_histogram.iter() {
        println!("Edges with weight {}: {}", count, freq);
    }


}