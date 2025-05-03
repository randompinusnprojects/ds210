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