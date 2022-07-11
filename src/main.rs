use std::io::{stdin, stdout, Write};

use edge::Edge;
use network::Network;
use node::Node;
use package::Package;
use train::Train;

pub mod edge;
pub mod network;
pub mod node;
pub mod package;
pub mod train;

fn get_input() -> (Vec<String>, Vec<String>, Vec<String>, Vec<String>) {
    let mut input_string = String::new();
    print!("Number of stations: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string).unwrap();
    if let Some('\n') = input_string.chars().next_back() {
        input_string.pop();
    }
    if let Some('\r') = input_string.chars().next_back() {
        input_string.pop();
    }
    let num_station: u64 = input_string.parse().unwrap();
    let mut node = vec![];
    for i in 0..num_station {
        input_string.clear();
        print!("Station {}: ", i + 1);
        stdout().flush().unwrap();
        stdin().read_line(&mut input_string).unwrap();
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
        }
        if let Some('\r') = input_string.chars().next_back() {
            input_string.pop();
        }
        node.push(input_string.clone());
    }

    println!("");
    input_string.clear();
    print!("Number of edges: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string).unwrap();
    if let Some('\n') = input_string.chars().next_back() {
        input_string.pop();
    }
    if let Some('\r') = input_string.chars().next_back() {
        input_string.pop();
    }
    let num_edge: u64 = input_string.parse().unwrap();
    let mut edge = vec![];
    for i in 0..num_edge {
        input_string.clear();
        print!("Edge {}: ", i + 1);
        stdout().flush().unwrap();
        stdin().read_line(&mut input_string).unwrap();
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
        }
        if let Some('\r') = input_string.chars().next_back() {
            input_string.pop();
        }
        edge.push(input_string.clone());
    }

    println!("");
    input_string.clear();
    print!("Number of deliveries to be performed: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string).unwrap();
    if let Some('\n') = input_string.chars().next_back() {
        input_string.pop();
    }
    if let Some('\r') = input_string.chars().next_back() {
        input_string.pop();
    }
    let num_package: u64 = input_string.parse().unwrap();
    let mut package = vec![];
    for i in 0..num_package {
        input_string.clear();
        print!("Package {}: ", i + 1);
        stdout().flush().unwrap();
        stdin().read_line(&mut input_string).unwrap();
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
        }
        if let Some('\r') = input_string.chars().next_back() {
            input_string.pop();
        }
        package.push(input_string.clone());
    }

    println!("");
    input_string.clear();
    print!("Number of trains: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut input_string).unwrap();
    if let Some('\n') = input_string.chars().next_back() {
        input_string.pop();
    }
    if let Some('\r') = input_string.chars().next_back() {
        input_string.pop();
    }
    let num_trains: u64 = input_string.parse().unwrap();
    let mut trains = vec![];
    for i in 0..num_trains {
        input_string.clear();
        print!("Trains {}: ", i + 1);
        stdout().flush().unwrap();
        stdin().read_line(&mut input_string).unwrap();
        if let Some('\n') = input_string.chars().next_back() {
            input_string.pop();
        }
        if let Some('\r') = input_string.chars().next_back() {
            input_string.pop();
        }
        trains.push(input_string.clone());
    }

    // println!("Nodes: {:?}", node);
    // println!("Edges: {:?}", edge);
    // println!("Packages: {:?}", package);
    // println!("Trains: {:?}", trains);
    (node, edge, package, trains)
}

fn create_network(
    node: Vec<String>,
    edge: Vec<String>,
    train: Vec<String>,
    package: Vec<String>,
) -> Network {
    let mut nodes = vec![];
    for n in node {
        nodes.push(Node::new(n));
    }
    let mut network = Network::new();
    network.initialize_node(nodes);
    let node_dict = network.clone();
    let mut edges = vec![];
    for e in edge {
        let edge_info = e.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        edges.push(Edge::new(
            edge_info[0].to_string(),
            node_dict.get_node(edge_info[1].clone()),
            node_dict.get_node(edge_info[2].clone()),
            edge_info[3].parse().unwrap(),
        ));
        let mut r_name = edge_info[0].to_string();
        r_name.push_str(" R");
        edges.push(Edge::new(
            r_name,
            node_dict.get_node(edge_info[2].clone()),
            node_dict.get_node(edge_info[1].clone()),
            edge_info[3].parse().unwrap(),
        ));
    }
    network.initialize_edge(edges);
    let mut trains = vec![];
    for t in train {
        let train_info = t.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        trains.push(Train::new(
            train_info[0].clone(),
            train_info[1].parse().unwrap(),
            node_dict.get_node(train_info[2].clone()),
        ));
    }
    network.initialize_train(trains);
    let mut packages = vec![];
    for p in package {
        let package_info = p.split(",").map(|x| x.to_string()).collect::<Vec<String>>();
        packages.push(Package::new(
            package_info[0].clone(),
            package_info[1].parse().unwrap(),
            node_dict.get_node(package_info[2].clone()),
            node_dict.get_node(package_info[3].clone()),
        ));
    }
    network.initialize_package(packages);
    network
}

// fn main() {
//     let station = vec!["A", "B", "C"]
//         .into_iter()
//         .map(|x| x.to_string())
//         .collect();
//     let edge = vec!["E1,A,B,30", "E2,B,C,10"]
//         .into_iter()
//         .map(|x| x.to_string())
//         .collect();
//     let deliveries = vec!["K1,5,A,C"]
//         .into_iter()
//         .map(|x| x.to_string())
//         .collect();
//     let train = vec!["Q1,6,B"].into_iter().map(|x| x.to_string()).collect();
//     let mut network = create_network(station, edge, train, deliveries);
//     network.calculate_shortest_distance_between_packages();
//     network.simulate();
//     network.print_history();
// }

// fn main() {
//     let station = vec![
//         "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
//     ]
//     .into_iter()
//     .map(|x| x.to_string())
//     .collect();
//     let edge = vec![
//         "E1,A,B,5",
//         "E2,A,C,7",
//         "E3,A,D,11",
//         "E4,A,E,17",
//         "E5,E,F,7",
//         "E6,F,G,6",
//         "E7,G,H,3",
//         "E8,H,I,1",
//         "E9,I,J,1",
//         "E10,E,J,10",
//         "E11,J,L,9",
//         "E12,J,K,12",
//         "E13,L,M,8",
//         "E14,L,N,10",
//         "E15,K,M,4",
//         "E16,N,O,13",
//         "E17,P,O,15",
//         "E18,E,P,9",
//         "E19,O,Q,20",
//         "E20,Q,R,22",
//         "E21,M,N,10",
//         "E22,H,L,15",
//     ]
//     .into_iter()
//     .map(|x| x.to_string())
//     .collect();
//     let deliveries = vec![
//         "K1,5,C,H",
//         "K2,16,D,M",
//         "K3,9,R,J",
//         "K4,15,H,Q",
//         "K5,10,K,H",
//         "K6,3,F,F",
//     ]
//     .into_iter()
//     .map(|x| x.to_string())
//     .collect();
//     let train = vec!["Q1,6,B", "Q2,20,Q", "Q3,15,J"]
//         .into_iter()
//         .map(|x| x.to_string())
//         .collect();
//     let mut network = create_network(station, edge, train, deliveries);
//     network.calculate_shortest_distance_between_packages();
//     network.simulate();
//     network.print_history();
// }

fn main() {
    let (station, edge, deliveries, train) = get_input();
    let mut network = create_network(station, edge, train, deliveries);
    network.calculate_shortest_distance_between_packages();
    network.simulate();
    println!("");
    println!("--------------------------------------------------------------------");
    println!("Output:");
    network.print_history();
}
