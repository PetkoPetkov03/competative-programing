#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn dfs<T>(graph: &HashMap<T, Vec<T>>, start: T) -> (Vec<String>, Vec<T>)
where
    T: Eq + Hash + Clone + std::fmt::Display,
{
    let mut stack = vec![(start.clone(), false)];
    let mut visited = HashSet::new();
    let mut log = vec![];
    let mut dfs_path = vec![];

    while let Some((node, is_out)) = stack.pop() {
        if is_out {
            log.push(format!("out {}", node));
        } else {
            if !visited.contains(&node) {
                log.push(format!("in {}", node));
                visited.insert(node.clone());
                dfs_path.push(node.clone());
                stack.push((node.clone(), true));

                if let Some(neighbors) = graph.get(&node) {
                    for neighbor in neighbors.iter().rev() {
                        if !visited.contains(neighbor) {
                            stack.push((neighbor.clone(), false));
                        }
                    }
                }
            }
        }
    }

    (log, dfs_path)
}

fn main() {
    let mut graph_int = HashMap::new();

    graph_int.insert(0, vec![1, 2]);
    graph_int.insert(1, vec![0, 3]);
    graph_int.insert(2, vec![0, 4]);
    graph_int.insert(3, vec![1]);
    graph_int.insert(4, vec![2]);

    let start_node_int = 0;
    let (log_int, path_int) = dfs(&graph_int, start_node_int);

    println!("Integer Graph DFS:");

    for entry in log_int {
        println!("{}", entry);
    }

    println!("DFS Path: {:?}", path_int);

    let mut graph_char = HashMap::new();

    graph_char.insert('A', vec!['B', 'C', 'D']);
    graph_char.insert('B', vec!['A', 'E', 'F']);
    graph_char.insert('C', vec!['A']);
    graph_char.insert('D', vec!['A']);
    graph_char.insert('E', vec!['B', 'G']);
    graph_char.insert('F', vec!['B', 'H']);
    graph_char.insert('G', vec!['E']);
    graph_char.insert('H', vec!['F']);

    let start_node_char = 'A';

    let (log_char, path_char) = dfs(&graph_char, start_node_char);

    println!("\nCharacter Graph DFS:");

    for entry in log_char {
        println!("{}", entry);
    }

    println!("DFS Path: {:?}", path_char);
}


