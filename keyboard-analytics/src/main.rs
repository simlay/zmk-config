use clap::Parser;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process;

/// Keyboard analytics tool: counts letter/bigram frequencies and computes ergonomic keyboard scores.
#[derive(Parser, Debug)]
#[command(name = "keyboard-analytics", about = "Analyze keyboard usage and compute ergonomic scores")]
struct Cli {
    /// Input text file to analyze
    #[arg(long)]
    input: Option<String>,

    /// Output file for single letter stats (default: single.csv)
    #[arg(long, default_value = "single.csv")]
    single_output: String,

    /// Output file for bigram stats (default: double.csv)
    #[arg(long, default_value = "double.csv")]
    double_output: String,

    /// First layer DTSI keymap file (left hand)
    #[arg(long)]
    first_dtsi: Option<String>,

    /// Second layer DTSI keymap file (right hand)
    #[arg(long)]
    second_dtsi: Option<String>,
}

/// Count single letter occurrences (case-insensitive).
/// Returns a HashMap of letter -> count.
fn count_letters(text: &str) -> HashMap<char, u64> {
    let mut counts: HashMap<char, u64> = HashMap::new();

    for ch in text.chars() {
        if ch.is_alphabetic() {
            let lower = ch.to_ascii_lowercase();
            *counts.entry(lower).or_insert(0) += 1;
        }
    }

    counts
}

/// Count two-letter combinations (case-insensitive).
/// Returns a HashMap of bigram -> count.
fn count_bigrams(text: &str) -> HashMap<String, u64> {
    let mut counts: HashMap<String, u64> = HashMap::new();
    let chars: Vec<char> = text.chars().filter(|c| c.is_alphabetic()).collect();

    for window in chars.windows(2) {
        let bigram: String = format!(
            "{}{}",
            window[0].to_ascii_lowercase(),
            window[1].to_ascii_lowercase()
        );
        *counts.entry(bigram).or_insert(0) += 1;
    }

    counts
}

/// Write single letter counts to CSV as letter,percentage.
fn write_single_csv(path: &str, counts: &HashMap<char, u64>, total: u64) {
    let mut lines: Vec<(char, f64)> = counts
        .iter()
        .map(|(&ch, &count)| (ch, count as f64 / total as f64))
        .collect();
    lines.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut csv_content = String::from("letter,percentage\n");
    for (ch, pct) in &lines {
        csv_content.push_str(&format!("{},{}\n", ch, pct));
    }

    fs::write(path, csv_content).unwrap_or_else(|e| {
        eprintln!("Error writing to {}: {}", path, e);
        process::exit(1);
    });

    println!("Wrote single letter stats to {}", path);
}

/// Write bigram counts to CSV as bigram,percentage.
fn write_double_csv(path: &str, counts: &HashMap<String, u64>, total: u64) {
    let mut lines: Vec<(String, f64)> = counts
        .iter()
        .map(|(k, &count)| (k.clone(), count as f64 / total as f64))
        .collect();
    lines.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut csv_content = String::from("bigram,percentage\n");
    for (bigram, pct) in &lines {
        csv_content.push_str(&format!("{},{}\n", bigram, pct));
    }

    fs::write(path, csv_content).unwrap_or_else(|e| {
        eprintln!("Error writing to {}: {}", path, e);
        process::exit(1);
    });

    println!("Wrote double letter stats to {}", path);
}

/// Represents a parsed DTSI keymap node (e.g., `l_hand { bindings = <...>; };`).
#[derive(Debug, Deserialize, Clone)]
pub struct DtsiKeymap {
    pub name: String,
    pub bindings: Vec<Vec<String>>,
}

/// Deserializer for DTSI keymap content.
/// Parses the DTSI format:
/// ```
/// l_hand {
///     bindings = <
/// &kp BSPC   &kp S     &kp N      &kp T     &kp H
/// &kp LCTRL  &kp A     &kp O      &kp E     &kp U
/// &kp LSHFT  &kp L     &kp R      &kp C     &kp G
///                             &kp LGUI  &sl 5     &kp SPACE
///     >;
/// };
/// ```
pub fn deserialize_dtsi(content: &str) -> DtsiKeymap {
    // Extract the node name: everything before `{`
    let node_name = extract_node_name(content);

    // Extract the bindings block between `<` and `>`
    let bindings_str = extract_bindings(content);

    // Parse the bindings into a 2D grid
    let bindings = parse_bindings_grid(&bindings_str);

    DtsiKeymap {
        name: node_name,
        bindings,
    }
}

fn extract_node_name(content: &str) -> String {
    // Find the first `{` and take everything before it as the node name
    if let Some(open_brace) = content.find('{') {
        let name = content[..open_brace].trim();
        // Remove any leading/trailing whitespace and newlines
        name.to_string()
    } else {
        String::from("unknown")
    }
}

fn extract_bindings(content: &str) -> String {
    // Find the first `<` after `bindings =` and the matching `>`
    let bindings_marker = content.find("bindings");
    if let Some(start) = bindings_marker {
        let after_marker = &content[start..];
        if let Some(open_angle) = after_marker.find('<') {
            if let Some(close_angle) = after_marker[open_angle..].find('>') {
                return after_marker[open_angle + 1..open_angle + close_angle].to_string();
            }
        }
    }
    String::new()
}

fn parse_bindings_grid(content: &str) -> Vec<Vec<String>> {
    let mut grid: Vec<Vec<String>> = Vec::new();

    // Split into lines, strip whitespace, and tokenize
    for line in content.lines() {
        let tokens: Vec<String> = line
            .split_whitespace()
            .filter(|t| !t.is_empty())
            .map(|t| t.to_string())
            .collect();

        if tokens.is_empty() {
            continue;
        }

        // Parse tokens into key cells.
        // The DTSI format uses &<behavior> <keyname> pairs:
        //   &kp BSPC   &kp S   &sl 5   &none
        // We extract just the key name from each pair, lowercased.
        let mut cells: Vec<String> = Vec::new();
        let mut i = 0;
        while i < tokens.len() {
            let token = &tokens[i];
            if token.starts_with('&') {
                // This is a behavior prefix like &kp, &sl, &none
                if token == "&none" {
                    // &none means an empty/transparent cell
                    cells.push(String::new());
                    i += 1;
                } else if i + 1 < tokens.len() {
                    // Next token is the key name, store lowercased
                    cells.push(tokens[i + 1].to_ascii_lowercase());
                    i += 2;
                } else {
                    // Orphan behavior prefix, treat as empty
                    cells.push(String::new());
                    i += 1;
                }
            } else {
                // Standalone token (shouldn't normally happen, but handle it)
                cells.push(token.to_ascii_lowercase());
                i += 1;
            }
        }

        grid.push(cells);
    }

    grid
}

/// Extract the first 3 rows and columns 1 through 5 (zero-indexed) from a grid.
/// Returns a Vec<Vec<T>> where each inner vec is a row slice.
pub fn extract_subgrid<T: Clone>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    grid.iter()
        .take(3)
        .map(|row| {
            row.iter()
                .skip(1)
                .take(5)
                .cloned()
                .collect()
        })
        .collect()
}

/// Calculate the total score of typing a word using the given score map.
/// The word is treated case-insensitively (both upper and lower case work).
/// Only alphabetic characters are scored; non-letters are skipped.
pub fn word_score(word: &str, scores: &HashMap<String, u8>) -> u8 {
    word.chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| {
            scores
                .get(&c.to_string())
                .copied()
                .unwrap_or(0)
        })
        .sum()
}

/// Represents a single letter entry from the CSV.
#[derive(Debug, Clone)]
pub struct LetterEntry {
    pub letter: String,
    pub percentage: f64,
}

/// Represents a bigram entry from the CSV.
#[derive(Debug, Clone)]
pub struct BigramEntry {
    pub bigram: String,
    pub percentage: f64,
}

/// Parse the single letter CSV file.
pub fn parse_single_csv(path: &str) -> Vec<LetterEntry> {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", path, e);
        process::exit(1);
    });

    let mut entries: Vec<LetterEntry> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            entries.push(LetterEntry {
                letter: parts[0].trim().to_string(),
                percentage: parts[1].trim().parse().unwrap_or(0.0),
            });
        }
    }
    entries
}

/// Parse the double letter (bigram) CSV file.
pub fn parse_double_csv(path: &str) -> Vec<BigramEntry> {
    let content = fs::read_to_string(path).unwrap_or_else(|e| {
        eprintln!("Error reading {}: {}", path, e);
        process::exit(1);
    });

    let mut entries: Vec<BigramEntry> = Vec::new();
    for (i, line) in content.lines().enumerate() {
        if i == 0 {
            continue; // skip header
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 2 {
            entries.push(BigramEntry {
                bigram: parts[0].trim().to_string(),
                percentage: parts[1].trim().parse().unwrap_or(0.0),
            });
        }
    }
    entries
}

/// Calculate the total keyboard score using letter scores, letter percentages, and bigram percentages.
///
/// Single letter score: sum(score(letter) * percentage) for each letter
/// Bigram score: sum((score(first) + score(second)) * percentage) for each bigram
/// Total: single letter score + bigram score
pub fn calculate_keyboard_score(
    scores: &HashMap<String, u8>,
    single_entries: &[LetterEntry],
    double_entries: &[BigramEntry],
) -> (f64, f64, f64) {
    // Single letter score
    let single_score: f64 = single_entries
        .iter()
        .map(|entry| {
            let score = scores
                .get(&entry.letter.to_lowercase())
                .copied()
                .unwrap_or(0) as f64;
            score * entry.percentage
        })
        .sum();

    // Bigram score
    let bigram_score: f64 = double_entries
        .iter()
        .map(|entry| {
            let chars: Vec<char> = entry.bigram.chars().collect();
            if chars.len() == 2 {
                let first_score = scores
                    .get(&chars[0].to_lowercase().to_string())
                    .copied()
                    .unwrap_or(0) as f64;
                let second_score = scores
                    .get(&chars[1].to_lowercase().to_string())
                    .copied()
                    .unwrap_or(0) as f64;
                (first_score + second_score) * entry.percentage
            } else {
                0.0
            }
        })
        .sum();

    let total = single_score + bigram_score;
    (single_score, bigram_score, total)
}

/// Compute keyscores for the first layer (left).
/// Score 0: A, O, E, U → row 1, cols 0-3
/// Score 1: S, N, T, H, L, R, C, G → rows 0,2 cols 0-3
/// Score 2: D, P, I → col 4 (any row)
pub fn compute_keyscores_first(grid: &[Vec<String>]) -> HashMap<String, u8> {
    let mut scores: HashMap<String, u8> = HashMap::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, key) in row.iter().enumerate() {
            let score = if col_idx == 4 {
                2
            } else if row_idx == 1 {
                0
            } else {
                1
            };
            scores.insert(key.clone(), score);
        }
    }

    scores
}

/// Compute keyscores for the second layer (right).
/// Score 3: SEMI, Y, K, J → row 1, cols 0-3
/// Score 4: Z, Q, M, W, SQT, COMMA, DOT, V → rows 0,2 cols 0-3
/// Score 5: F, X, B → col 4 (any row)
pub fn compute_keyscores_second(grid: &[Vec<String>]) -> HashMap<String, u8> {
    let mut scores: HashMap<String, u8> = HashMap::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, key) in row.iter().enumerate() {
            let score = if col_idx == 4 {
                5
            } else if row_idx == 1 {
                3
            } else {
                4
            };
            scores.insert(key.clone(), score);
        }
    }

    scores
}

fn main() {
    let cli = Cli::parse();

    // Initialize variables that may be used later
    let mut single_counts: HashMap<char, u64> = HashMap::new();
    let mut single_total: u64 = 0;
    let mut double_counts: HashMap<String, u64> = HashMap::new();
    let mut double_total: u64 = 0;
    let mut first_scores: Option<HashMap<String, u8>> = None;

    // Handle keymap file analysis
    if let Some(ref input) = cli.input {
        let text = fs::read_to_string(input).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", input, e);
            process::exit(1);
        });

        // Count single letters
        single_counts = count_letters(&text);
        single_total = single_counts.values().sum();

        // Count bigrams
        double_counts = count_bigrams(&text);
        double_total = double_counts.values().sum();

        // Write outputs
        if single_total > 0 {
            write_single_csv(&cli.single_output, &single_counts, single_total);
        } else {
            eprintln!("No letters found in input text");
            process::exit(1);
        }

        if double_total > 0 {
            write_double_csv(&cli.double_output, &double_counts, double_total);
        } else {
            eprintln!("No bigrams found in input text");
            process::exit(1);
        }
    }

    // Handle first layer keymap
    if let Some(ref first_path) = cli.first_dtsi {
        let content = fs::read_to_string(first_path).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", first_path, e);
            process::exit(1);
        });

        let keymap = deserialize_dtsi(&content);
        let subgrid = extract_subgrid(&keymap.bindings);
        let scores = compute_keyscores_first(&subgrid);

        println!("First keymap ({}):", keymap.name);
        for (i, row) in subgrid.iter().enumerate() {
            println!("  Row {}: {:?}", i, row);
        }
        println!("First scores (key → score):");
        for (key, score) in &scores {
            println!("  {} → {}", key, score);
        }

        let json = serde_json::to_string_pretty(&scores).unwrap();
        println!("First scores JSON:\n{}", json);

        first_scores = Some(scores);
    }

    // Handle second layer keymap
    if let Some(ref second_path) = cli.second_dtsi {
        let content = fs::read_to_string(second_path).unwrap_or_else(|e| {
            eprintln!("Error reading {}: {}", second_path, e);
            process::exit(1);
        });

        let keymap = deserialize_dtsi(&content);
        let subgrid = extract_subgrid(&keymap.bindings);
        let scores = compute_keyscores_second(&subgrid);

        println!("Second keymap ({}):", keymap.name);
        for (i, row) in subgrid.iter().enumerate() {
            println!("  Row {}: {:?}", i, row);
        }
        println!("Second scores (key → score):");
        for (key, score) in &scores {
            println!("  {} → {}", key, score);
        }

        let json = serde_json::to_string_pretty(&scores).unwrap();
        println!("Second scores JSON:\n{}", json);

        if let Some(ref mut merged) = first_scores {
            merged.extend(scores);
            println!("\nMerged scores (first + second):");
            for (key, score) in &*merged {
                println!("  {} → {}", key, score);
            }
            let json = serde_json::to_string_pretty(merged).unwrap();
            println!("Merged scores JSON:\n{}", json);
        }
    }

    // Demonstrate word scoring with merged scores
    if let Some(ref scores) = first_scores {
        let test_words = vec!["THERE", "HELLO", "VIM", "EAT", "THE"];
        println!("\nWord scores (merged first + second):");
        for word in &test_words {
            let score = word_score(word, scores);
            println!("  {} → {}", word, score);
        }
    }

    // Calculate total keyboard score from in-memory usage data (if both input and keymaps provided)
    if let Some(ref scores) = first_scores {
        let single_entries: Vec<LetterEntry> = single_counts
            .iter()
            .map(|(&ch, &count)| {
                let percentage = count as f64 / single_total as f64;
                LetterEntry {
                    letter: ch.to_ascii_lowercase().to_string(),
                    percentage,
                }
            })
            .collect();

        let double_entries: Vec<BigramEntry> = double_counts
            .iter()
            .map(|(bigram, &count)| {
                let percentage = count as f64 / double_total as f64;
                BigramEntry {
                    bigram: bigram.clone(),
                    percentage,
                }
            })
            .collect();

        let (single_score, bigram_score, total) =
            calculate_keyboard_score(scores, &single_entries, &double_entries);

        println!("\nKeyboard score (merged scores + usage data):");
        println!("  Single letter score: {:.6}", single_score);
        println!("  Bigram score:        {:.6}", bigram_score);
        println!("  Total score:         {:.6}", total);
    }
}
