# Elliptic Bitcoin Transaction Analysis (Rust)

---

## A. Project Overview

**Goal:**
Detect potential money laundering behavior on the Elliptic Bitcoin dataset by analyzing **temporal transaction paths** and identifying **hub-like intermediaries** reused disproportionately in illicit flows compared to licit ones.

**Key Hypothesis:**
Nodes reused in multiple time-respecting transaction paths from illicit start points—but not from licit ones—may represent mixers or anonymization services.

**Dataset:**

* [Elliptic dataset on Kaggle](https://www.kaggle.com/datasets/ellipticco/elliptic-data-set)
* Files used:

  * `elliptic_txs_edgelist.csv` — directed transaction graph
  * `elliptic_txs_classes.csv` — label file (licit, illicit, unknown)
  * `elliptic_txs_features.csv` — timestamps for each transaction
* Total size: \~650MB (features file too large for GitHub)

---

## B. Data Processing

**Loading Method:**
All CSVs are parsed using custom Rust module `fileread.rs`:

* Graph: `HashMap<String, HashSet<String>>` (from edgelist)
* Labels and timestamps: `HashMap<String, String>` → then parsed into `usize`

**Cleaning / Transformation:**

* Paths are pruned to only respect **non-decreasing timestamps**
* Maximum path length and number of paths per (start, end) pair are capped to avoid exponential blowup
* Nodes are sampled from reachable sets rather than full enumeration to limit memory pressure

---

## C. Code Structure

### Modules

* `fileread.rs` — reads edgelist, features, labels into appropriate Rust types.
* `dfsstuff.rs` — DFS-based path analysis functions (reachable node discovery, path enumeration, stats).
* `main.rs` — sampling, scoring, orchestration of full experiment.

---

### Key Functions and Logic

#### `dfsstuff::dfs_summary`

* **Purpose:** Count all time-respecting paths from start to target (up to limits).
* **Inputs:** graph, timestamps, start/target node, depth & path caps
* **Outputs:** `HashMap<(start, target), (path_count, total_depth)>`
* **Key logic:** DFS with temporal monotonicity + visited set + early cutoff on depth/path count.

#### `dfsstuff::dfs_collect_paths`

* **Purpose:** Actually store full paths for top start-target pairs.
* **Used for:** Detecting reused intermediaries.

#### `theory_tester`

* **Purpose:** From sampled illicit or licit nodes, compute top hub-like intermediaries.
* **Steps:**

  * Find reachable high-outdegree nodes.
  * Sample (start, target) pairs.
  * Summarize path stats and collect full paths for top pairs.
  * Tally reused nodes in middle of paths.

#### `compute_mixer_data`

* Computes score = `illicit_count / (licit_count + 1.0)` for each reused node.

#### `summarize_scores`

* Aggregates scores over multiple samplings.
* Reports mean, stddev, 95% confidence interval for each candidate node.

---

### Main Workflow

1. Sample 100 illicit and 100 licit nodes.
2. For each:

   * Compute top reused intermediaries.
   * Tally node reuse frequency.
3. Calculate mixer score across `num_runs` (e.g. 10).
4. Rank all nodes by average score and confidence.

---

## D. Tests

Use `cargo test` to run:

```bash
cargo test
```

### Included Tests:

* `test_dfs_summarize`: verifies path counting in toy DAG
* `test_dfs_summarize_2`: ensures behavior when all timestamps are equal
* **Assertions:** total path count and total depth are both checked (e.g., count = 3, depth = 12)

---

## E. Results

### Example Output:

```
[illicit] 94407937 → 156055502 has 49 full paths
[licit] 355009655 → 355009675 has 10 full paths

[illicit] High-degree hub reused: 155576355 (52 times)
[licit] High-degree hub reused: 355009675 (1 times)

Top 20 Mixer Candidates by Mean Score:
Node            Mean     StdDev       95% CI Low          95% CI High
155576355         4.2        1.5             3.5                  4.9
135011010         3.8        1.2             3.3                  4.3
...
```

**Interpretation:**
High score + narrow confidence range indicates stable signal across samples.
Illicit paths frequently share intermediaries absent in licit samples → potential mixers.

---

## F. Usage Instructions

### Build:

```bash
cargo build --release
```

### Run:

```bash
cargo run
```

- Adjust `num_runs`, `sample_size `, `max_depth `, `max_path` as needed, will vary runtime a **lot**.
-  /transactionbased is the code that implements above algorithm, which works for transaction based graphs.
-  /accountbased has failed attempt of implementing algorithm for account based graphs. 

// Tried mapping transaction based graphs into account based graphs but failed to do so.

**Expected Runtime:**
\~2–4 minutes depending on machine (due to DFS with depth limits and sampling loop).

**Dependencies:**

* `rand`
---

## G. AI-Assistance Disclosure and Citations

### ChatGPT Used For:

* Structuring path counting and limiting logic
* Code organization suggestions
* Statistical score interpretation and CI calculation
* Minor debugging assistance during DFS development

**Verification:**
All generated snippets were manually reviewed, benchmarked with toy graphs, and confirmed via `cargo test`.

```bash
fn summarize_scores(score_map: HashMap<String, Vec<f64>>) -> Vec<MixerStats> { // GPT
    let mut result = vec![];
    for (node, scores) in score_map {
        let n = scores.len() as f64;
        let mean = scores.iter().copied().sum::<f64>() / n;
        let variance = scores.iter().map(|s| (s - mean).powi(2)).sum::<f64>() / n; // equation for variance is \sum(x - mean)^2
        let stddev = variance.sqrt();
        let ci_margin = 1.96 * stddev / n.sqrt(); // This is computing confidence interval, 1.96 is used for 95% CI

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
```

---

## H. Known Limitations

* **Sampling Bias:** Still based on reachable subgraphs; might miss global patterns.
* **Path Loss on Abstraction:** Account-level generalizations may obscure transactional detail.
* **Reused Node Interpretation:** Some high-score nodes could be legitimate hubs (exchanges, payment aggregators).
