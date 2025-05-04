use plotters::prelude::*;
use std::error::Error;
use std::collections::{HashMap, HashSet};
use std::vec;
use std::io::{BufReader, BufRead};
use std::fs::File;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rand::prelude::IndexedRandom;

mod fileread;
mod dfsstuff;

fn top_outdegree_calculator(
    graph: &HashMap<String, HashSet<String>>,
    limit: usize,
) -> Vec<String> {
    let mut degrees: Vec<_> = graph
        .iter()
        .map(|(node, neighbors)| (node.clone(), neighbors.len()))
        .collect();
    degrees.sort_by(|a, b| b.1.cmp(&a.1));
    degrees.into_iter().take(limit).map(|(node, _)| node).collect()
}

fn reachable_calculator(
    graph: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    start_nodes: &Vec<String>,
    depth: usize,
) -> HashSet<String> {
    let mut reachable = HashSet::new();
    let mut visited = HashSet::new();
    for start in start_nodes {
        dfsstuff::dfs_collect_reachable(
            graph,
            timestamps,
            start,
            1,
            &mut visited,
            &mut reachable,
            depth,
        );
    }
    reachable
}

fn sampler(nodes: Vec<String>, limit: usize) -> Vec<String> {
    let mut rng = thread_rng();
    let sample: Vec<String> = nodes
        .choose_multiple(&mut rng, limit)
        .cloned()
        .collect();
    sample
}

fn theory_tester(
    edges: &HashMap<String, HashSet<String>>,
    timestamps: &HashMap<String, usize>,
    start_nodes: &Vec<String>,
    label: &str,  // "illicit" or "licit"
) -> HashMap<String, usize> {
    let mut rng = thread_rng();

    let reachable = reachable_calculator(edges, timestamps, start_nodes, 10); // adjust if needed
    println!("[{}] Reachable count: {}", label, reachable.len());

    let mut degrees_reachable: Vec<(String, usize)> = reachable
        .iter()
        .filter_map(|node| edges.get(node).map(|neighbors| (node.clone(), neighbors.len())))
        .collect();

    degrees_reachable.sort_by(|a, b| b.1.cmp(&a.1));

    let top_outdegree_reachable: Vec<String> = degrees_reachable
        .into_iter()
        .take(100)
        .map(|(node, _)| node)
        .collect();

    let sampled_targets = sampler(top_outdegree_reachable.clone(), 100);

    let stats = dfsstuff::summarize_paths_to_targets(edges, timestamps, start_nodes, &sampled_targets, 15, 100); // adjust if needed
    
    let mut stat_entries: Vec<_> = stats.iter().collect();
    stat_entries.sort_by(|a, b| b.1.0.cmp(&a.1.0));  // by path count
    let top_pairs: Vec<(&(String, String), &(usize, usize))> = stat_entries.into_iter().take(10).collect();

    let mut all_paths = Vec::new();

    for ((start, target), _) in top_pairs {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
    
        dfsstuff::dfs_collect_paths(
            edges, timestamps,
            start, target,
            &mut path, &mut all_paths, &mut visited,
            1, 20,
        );
    
        println!("[{}] {} → {} has {} full paths", label, start, target, all_paths.len());
    }

    let mut node_freq = HashMap::new();
    for path in &all_paths {
        for node in &path[1..path.len() - 1] {
            *node_freq.entry(node.clone()).or_insert(0) += 1;
        }
    }

    let high_degree_set: HashSet<String> = top_outdegree_reachable.iter().cloned().collect();

    for (node, freq) in &node_freq {
        // println!("[{}] Non High-degree hub reused: {} ({} times)", label, node, freq);
        if high_degree_set.contains(node) {
            println!("[{}] High-degree hub reused: {} ({} times)", label, node, freq); // comparison between intermediary and high-deg nodes
        }
    }

    println!("[{}] Done.\n", label);
    node_freq
}

struct MixerStats {
    node: String,
    scores: Vec<f64>,
    mean: f64,
    stddev: f64,
    ci_low: f64,
    ci_high: f64,
}

fn compute_mixer_data(
    node_freq_illicit: &HashMap<String, usize>,
    node_freq_licit: &HashMap<String, usize>,
) -> Vec<(String, u32, u32, f64)> {
    let mut mixer_data = Vec::new();
    let all_nodes: std::collections::HashSet<_> = 
        node_freq_illicit.keys()
        .chain(node_freq_licit.keys())
        .collect();

    for node in all_nodes {
        let illicit = *node_freq_illicit.get(node).unwrap_or(&0) as u32;
        let licit = *node_freq_licit.get(node).unwrap_or(&0) as u32;
        let score = illicit as f64 / (licit as f64 + 1.0);  // Avoid division by zero
        mixer_data.push((node.clone(), licit, illicit, score));
    }

    mixer_data
}

fn summarize_scores(score_map: HashMap<String, Vec<f64>>) -> Vec<MixerStats> { // GPT
    let mut result = vec![];
    for (node, scores) in score_map {
        let n = scores.len() as f64;
        let mean = scores.iter().copied().sum::<f64>() / n;
        let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / n;
        let stddev = variance.sqrt();
        let ci_margin = 1.96 * stddev / n.sqrt(); // 95% CI

        result.push(MixerStats {
            node,
            scores,
            mean,
            stddev,
            ci_low: mean - ci_margin,
            ci_high: mean + ci_margin,
        });
    }

    result.sort_by(|a, b| b.mean.partial_cmp(&a.mean).unwrap());
    result
}


fn main() {
    println!("Reading.");
    let labels = fileread::read_to_hashmap("../../elliptic_txs_classes.csv");
    println!("Reading..");
    let edges = fileread::read_file_directed("../../elliptic_txs_edgelist.csv");
    println!("Reading...");
    let raw_timestamps = fileread::read_to_hashmap("../../elliptic_txs_features.csv");

    let timestamps: HashMap<String, usize> = raw_timestamps
        .into_iter()
        .filter_map(|(k, v)| v.parse::<usize>().ok().map(|ts| (k, ts)))
        .collect();

    println!("Finished reading!");

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

    println!("Found illicit nodes");

    let mut score_map: HashMap<String, Vec<f64>> = HashMap::new();
    let num_runs = 10; // adjust if needed

    for _ in 0..num_runs {
        let sampled_illicit_nodes = sampler(illicit_nodes.clone(), 100);
        let sampled_licit_nodes = sampler(licit_nodes.clone(), 100);
    
        let node_freq_illicit = theory_tester(&edges, &timestamps, &sampled_illicit_nodes, "illicit");
        let node_freq_licit = theory_tester(&edges, &timestamps, &sampled_licit_nodes, "licit");
    
        let mixer_data = compute_mixer_data(&node_freq_illicit, &node_freq_licit);

        for (node, licit, illicit, score) in mixer_data {
            score_map.entry(node).or_default().push(score);
        }
    }

    println!("Almost there!");

    let final_stats = summarize_scores(score_map);

    println!("\nTop 20 Mixer Candidates by Mean Score:");
    println!("{:<15} {:>10} {:>10} {:>15} {:>20}", 
        "Node", "Mean", "StdDev", "95% CI Low", "95% CI High");

    for stat in final_stats.iter().take(20) {
        println!("{:<15} {:>10.2} {:>10.2} {:>15.2} {:>20.2}", 
            stat.node, stat.mean, stat.stddev, stat.ci_low, stat.ci_high);
    }

    
}

#[test]
fn test_dfs_summarize() {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    graph.insert("A".into(), ["B"].iter().map(|s| s.to_string()).collect());
    graph.insert("B".into(), ["C", "F"].iter().map(|s| s.to_string()).collect());
    graph.insert("C".into(), ["F", "D"].iter().map(|s| s.to_string()).collect());
    graph.insert("D".into(), ["F"].iter().map(|s| s.to_string()).collect());
    graph.insert("F".into(), HashSet::new());

    let mut ts: HashMap<String, usize> = HashMap::new();
    ts.insert("A".into(), 1);
    ts.insert("B".into(), 2);
    ts.insert("C".into(), 3);
    ts.insert("D".into(), 4);
    ts.insert("F".into(), 5);

    let start_nodes = vec!["A".to_string()];
    let end_nodes = vec!["F".to_string()];


    let stats_test = dfsstuff::summarize_paths_to_targets(&graph, &ts, &start_nodes, &end_nodes);
    for ((start, end), (count, total_depth)) in &stats_test {
        let avg_depth = *total_depth as f64 / *count as f64;
        println!("{} → {}: {} paths, avg depth {:.2}", start, end, count, avg_depth);
    }

    let result = stats_test.get(&("A".to_string(), "F".to_string()));
    
    let (count, total_depth) = result.unwrap();
    assert_eq!(*count, 3);
    assert_eq!(*total_depth, 12);
}

#[test] // same timestamp
fn test_dfs_summarize_2() {
    let mut graph: HashMap<String, HashSet<String>> = HashMap::new();
    graph.insert("A".into(), ["B"].iter().map(|s| s.to_string()).collect());
    graph.insert("B".into(), ["C", "F"].iter().map(|s| s.to_string()).collect());
    graph.insert("C".into(), ["F", "D"].iter().map(|s| s.to_string()).collect());
    graph.insert("D".into(), ["F"].iter().map(|s| s.to_string()).collect());
    graph.insert("F".into(), HashSet::new());

    let mut ts: HashMap<String, usize> = HashMap::new();
    for node in ["A", "B", "C", "D", "F"] {
        ts.insert(node.to_string(), 1);
    }

    let start_nodes = vec!["A".to_string()];
    let end_nodes = vec!["F".to_string()];


    let stats_test = dfsstuff::summarize_paths_to_targets(&graph, &ts, &start_nodes, &end_nodes);
    for ((start, end), (count, total_depth)) in &stats_test {
        let avg_depth = *total_depth as f64 / *count as f64;
        println!("{} → {}: {} paths, avg depth {:.2}", start, end, count, avg_depth);
    }
    let result = stats_test.get(&("A".to_string(), "F".to_string()));

    let (count, total_depth) = result.unwrap();
    assert_eq!(*count, 3);
    assert_eq!(*total_depth, 12);
}