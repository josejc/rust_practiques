mod prelude {
    pub use std::collections::HashMap;
}

use prelude::*;

fn new_graph() -> std::collections::HashMap<String, HashMap<String, isize>> {
    let mut graph: HashMap<String, HashMap<String, isize>> = HashMap::new();

    // Insert Nodes and his weigh
    let mut neighbors: HashMap<String, isize> = HashMap::new();
    neighbors.insert("A".to_string(), 6);
    neighbors.insert("B".to_string(), 2);
    graph.insert("S".to_string(), neighbors);

    let mut neighbors = HashMap::new();
    neighbors.insert("F".to_string(), 1);
    graph.insert("A".to_string(), neighbors);

    let mut neighbors = HashMap::new();
    neighbors.insert("A".to_string(), 3);
    neighbors.insert("F".to_string(), 5);
    graph.insert("B".to_string(), neighbors);

    let neighbors = HashMap::new();
    graph.insert("F".to_string(), neighbors);

    graph 
}

fn ini_dijkstra(graph: &HashMap<String, HashMap<String, isize>>) -> (HashMap<String, isize>, HashMap<String, String>) {
    let mut costs: HashMap<String, isize> = HashMap::new();
    let mut parents: HashMap<String, String> = HashMap::new();

    if let Some(c) = graph.get(&"S".to_string()) {
        costs = c.clone();
    }
    //add node "fin" with weigh infinity if doesn't exists
    if !costs.contains_key(&"F".to_string()) {
        costs.insert("F".to_string(), isize::MAX);
    }

    // Initialice parents
    for value in costs.iter() {
        // value.0 = node, value.1 = cost
        if *value.1 != isize::MAX {
            parents.insert(value.0.to_string(), "S".to_string());
        } else {
            parents.insert(value.0.to_string(), "".to_string());
        }
    }

    (costs, parents)
}

//p.139 Grokking algorithms
fn find_lowest_cost_node(costs: &HashMap<String, isize>, procesed: &Vec<String>) -> String {
    let mut lowest_cost = isize::MAX;
    let mut lowest_cost_node = "".to_string();

    for value in costs.iter() {
        // value.0 = node, value.1 = cost
        if *value.1 < lowest_cost && !procesed.contains(value.0) {
            lowest_cost = *value.1;
            lowest_cost_node = value.0.clone();
        }
    }

    lowest_cost_node
}

//p.134 Grokking algoritms
fn main() {
    let g = new_graph();
    let tuple: (HashMap<String, isize>, HashMap<String, String>) = ini_dijkstra(&g);
    let mut costs: HashMap<String, isize> = tuple.0;
    let mut parents: HashMap<String, String> = tuple.1;
    let mut procesed: Vec<String> = vec![];
    let mut cost: isize;
    let mut neighbors: HashMap<String, isize>;

    println!("Graph: {:?}", g);
    println!("Costs: {:?}", costs);
    println!("Parents: {:?}", parents);
    println!("Node low cost: {}", find_lowest_cost_node(&costs, &procesed));

    let mut node: String = find_lowest_cost_node(&costs, &procesed);
    while !node.is_empty() {
        if let Some(c) = costs.get(&node.to_string()) {
            cost = *c;
            if let Some(n) = g.get(&node.to_string()) {
                neighbors = n.clone();
                for n in neighbors.iter() {
                    let new_cost = cost + *n.1;
                    // Check if it's cheaper to get to this neigbor by going through this node
                    if let Some(c) = costs.get(&n.0.to_string()) {
                        if *c > new_cost {
                            costs.insert(n.0.to_string(), new_cost);
                            parents.insert(n.0.to_string(), node.to_string());
                        }
                    }
                }
                procesed.push(node);
                node = find_lowest_cost_node(&costs, &procesed);
            }
        }
    }
    println!("---Costs: {:?}", costs);
    println!("---Parents: {:?}", parents);

    // The solution, returning from the end node
    // Use Vec like a Stack
    let mut sol_node: Vec<String> = vec![];
    let mut sol_weight: Vec<isize> = vec![];
    
    node = "F".to_string();
    sol_node.push(node.clone());
    while node != "S" {
        if let Some (c) = costs.get(&node.to_string()) {
            sol_weight.push(*c);
        }
        if let Some(p) = parents.get(&node.to_string()) {
            node = p.to_string();
            sol_node.push(node.clone());
        }
    }
    let sol_node_aux = sol_node.clone();
    // Now print the Solution ;-)
    println!("The path from the initial node with the accumulated weight"); 
    while node != "F"{
        if let Some(n) = sol_node.pop() {
            node = n.to_string();
            print!("{}", node);
        }
        if let Some(c) = sol_weight.pop() {
            print!("---({})--->", c);
        }
    }
    println!("");
    
    sol_node = sol_node_aux;
    println!("The path from the initial node with the weight per node"); 
    let mut node_parent: String;
    if let Some(n) = sol_node.pop() {
        node = n.to_string();
    }
    while node != "F"{
        if let Some(n) = sol_node.pop() {
            node_parent = node;
            node = n.to_string();
            print!("{}", node_parent);
            if let Some(n) = g.get(&node_parent) {
                if let Some(c) = n.get(&node) {
                    print!("---({})--->", c);
                }
            }
        }
    }
    println!("{}", node);
    
}
