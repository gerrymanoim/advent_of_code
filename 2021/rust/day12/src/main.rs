use utils::stdin_to_str_vec;
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;

fn paths_to_graph(v: Vec<String>) -> Graph {
    let mut graph: Graph = HashMap::new();
    for path in v.iter() {
        let (start, end) = path.split_once('-').unwrap();
        graph.entry(start.to_string()).or_default().push(end.to_string());
        graph.entry(end.to_string()).or_default().push(start.to_string());
    }

    graph
}

fn visitor(current_node: String, mut visited_path: Vec<String>, g: &Graph, mut results: &mut Vec<(String, usize)>) {
    // TODO do this with much less copying
    visited_path.push(current_node.clone());
    if current_node == "end" {
        results.push((visited_path.join("->"), visited_path.len()))
    } else {
        // println!("{}", current_node);
        let next_paths = &g[&current_node];
        for next_path in next_paths.iter() {
            if next_path == "start" {
                continue;
            }
            if *next_path != next_path.to_lowercase() || !visited_path.contains(next_path) {
                // safe to keep going
                visitor(next_path.to_owned(), visited_path.clone(), g, results);
            }
        }
    }
}

fn part_1(g: &Graph) -> usize {
    let mut results = Vec::<(String, usize)>::new();
    let mut visited_path = Vec::<String>::new();

    visitor("start".to_string(), visited_path, g, &mut results);
    // println!("{:?}", results);
    results.len()
}

fn visitor_part_2(current_node: String, mut visited_path: Vec<String>, g: &Graph, mut results: &mut Vec<(String, usize)>, double_visit: bool) {
    // TODO do this with much less copying
    visited_path.push(current_node.clone());
    if current_node == "end" {
        results.push((visited_path.join("->"), visited_path.len()))
    } else {
        // println!("{}", current_node);
        let next_paths = &g[&current_node];
        for next_path in next_paths.iter() {
            if next_path == "start" {
                continue;
            } else if *next_path != next_path.to_lowercase() {
                visitor_part_2(next_path.to_owned(), visited_path.clone(), g, results, double_visit);
            } else if !visited_path.contains(next_path) {
                // safe to keep going
                visitor_part_2(next_path.to_owned(), visited_path.clone(), g, results, double_visit);
            } else if !double_visit {
                visitor_part_2(next_path.to_owned(), visited_path.clone(), g, results, !double_visit);
            }
        }
    }
}


fn part_2(g: &Graph) -> usize {
    let mut results = Vec::<(String, usize)>::new();
    let mut visited_path = Vec::<String>::new();

    visitor_part_2("start".to_string(), visited_path, g, &mut results, false);
    // println!("{:?}", results);
    results.len()
}

fn main() {
    let raw_input = stdin_to_str_vec();
    let g = paths_to_graph(raw_input);
    println!("{:?}", part_1(&g));
    println!("{:?}", part_2(&g));
}

#[test]
fn examples() {
    let raw_input = "start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end".lines().map(|l|l.trim().to_string()).collect();
    let g = paths_to_graph(raw_input);
    println!("{:?}", g);
    assert_eq!(part_1(&g), 10);
    assert_eq!(part_2(&g), 36);
}
