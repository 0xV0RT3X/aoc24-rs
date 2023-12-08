use num::integer::lcm;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    left: String,
    right: String,
}

fn parse_input(input: &str) -> (Vec<char>, HashMap<String, Node>) {
    let mut lines = input.lines();

    let instructions: Vec<char> = lines.next().unwrap_or_default().trim().chars().collect();

    let mut positions = HashMap::new();

    for line in lines {
        if let Some((key, node)) = parse_line(line) {
            positions.insert(key, node);
        }
    }

    (instructions, positions)
}

fn parse_line(line: &str) -> Option<(String, Node)> {
    let parts: Vec<&str> = line.trim().split(" = ").collect();
    if parts.len() == 2 {
        let key = parts[0].to_string();
        let values: Vec<&str> = parts[1][1..parts[1].len() - 1].split(", ").collect();
        let node = Node {
            left: values[0].to_string(),
            right: values[1].to_string(),
        };
        Some((key, node))
    } else {
        None
    }
}

fn find_z_nodes(positions: &HashMap<String, Node>) -> Vec<&String> {
    positions.keys().filter(|key| key.ends_with('Z')).collect()
}

fn find_a_nodes(positions: &HashMap<String, Node>) -> Vec<String> {
    positions
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect()
}

fn node_steps(
    a_node: &String,
    z_nodes: &Vec<&String>,
    positions: &HashMap<String, Node>,
    instructions: &Vec<char>,
) -> usize {
    let mut steps = 0;
    let mut current_node = a_node.clone();

    while !z_nodes.contains(&&current_node) {
        let Node { left, right } = positions.get(&current_node).expect("Node not found");

        match instructions[steps % instructions.len()] {
            'R' => current_node = right.clone(),
            'L' => current_node = left.clone(),
            _ => {}
        }

        steps += 1;
    }

    steps
}

fn main() {
    let (instructions, positions) = parse_input(include_str!("../input.txt"));

    let z_nodes = find_z_nodes(&positions);
    let mut a_nodes = find_a_nodes(&positions);

    let mut lens = Vec::new();

    for a_node in a_nodes {
        lens.push(node_steps(&a_node, &z_nodes, &positions, &instructions));
    }

    let result = lens.iter().fold(1, |acc, &len| lcm(acc, len));

    println!("{:?}", result);
}
