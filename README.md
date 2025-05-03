# Elliptic Bitcoin Transaction Analysis (Rust)

## A. Project Overview

**Goal:**  
To analyze Bitcoin transaction patterns from the [Elliptic dataset](https://www.kaggle.com/datasets/ellipticco/elliptic-data-set) to detect signs of money laundering or non-standard financial behavior—particularly via **intermediary reuse** and **account structure reconstruction**.

**Dataset:**  
- Source: Elliptic dataset (elliptic_txs_classes.csv, elliptic_txs_edgelist.csv, elliptic_txs_features.csv)
- Size: ~650MB (features file too large for GitHub)

---

## B. Data Processing

**Loading Method:**  
Custom `fileread.rs` module that:
- Reads CSVs into `HashMap<String, HashSet<String>>` (edgelist) or `HashMap<String, String>` (labels, timestamps).

**Transformations Applied:**
- Converted transaction-based graph into an **inferred account-based graph**, using timestamp-filtered paths.
- Filtered transaction paths to respect temporal ordering (e.g., txB cannot follow txA if its timestamp is earlier).

---

## C. Code Structure

### Modules

- `fileread.rs`: CSV loading utilities.
- `main.rs`: Path traversal, timestamp filtering, account assignment, analysis.

### Key Functions

#### `collect_maximal_paths_time_filtered`

- **Purpose:**  
  Traverse all paths forward in time while preventing temporal inconsistencies and node revisits.
- **Input:**  
  Graph, timestamp map, list of start nodes.
- **Output:**  
  Vector of transaction paths.
- **Core Logic:**  
  DFS traversal while enforcing monotonic increase in timestamps.

#### `assign_accounts_from_paths`

- **Purpose:**  
  Assign synthetic account IDs to transactions based on sequential path order.
- **Input:**  
  Paths, timestamp map.
- **Output:**  
  `HashMap<String, TxEdge>` where each transaction gets a (start_account, end_account).
- **Core Logic:**  
  Reuses end account of previous tx as start of current one. Allocates new accounts otherwise.

#### `build_weighted_account_graph`

- **Purpose:**  
  Aggregate account-to-account edges from assigned transaction mappings.
- **Output:**  
  `HashMap<usize, Vec<(usize, usize)>>` representing adjacency list with edge weight (repetition count).

---

## E. Results

- Top reused intermediary nodes were identifiable after path reconstruction.
- Edge weight distribution showed most account-level edges were used only once.
- Larger reuse patterns only emerge in specific clusters of illicit transactions.

## F. Usage Instructions

To run the project:

TBD

## G. Current Problems to Solve

### Loss of Path Fidelity Post-Account Assignment

- Once transaction paths are transformed into an account graph, the exact temporal and structural fidelity of paths is lost.
- Any subsequent intermediary detection or motif analysis done purely on the account-level graph may reflect paths that were not actually possible.

### Memory Constraints for Full Graph Traversal

- Enumerating full transaction paths (especially from all illicit nodes) leads to OOM errors.
- Currently mitigated via batching, but limits global path-based analyses.

### Indistinguishable Intermediaries

#### Intermediary reuse patterns may arise due to:

- Legitimate batching (e.g. exchanges or mixers)

- Illicit aggregation

Without additional address-level metadata, disambiguating these scenarios is difficult.
