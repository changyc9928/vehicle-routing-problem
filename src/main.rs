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

fn main() {
    let station = vec!["A", "B", "C"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let edge = vec!["E1,A,B,30", "E2,B,C,10"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let deliveries = vec!["K1,5,A,C"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let train = vec!["Q1,6,B"].into_iter().map(|x| x.to_string()).collect();
    let mut network = create_network(station, edge, train, deliveries);
    network.calculate_shortest_distance_between_packages();
    network.simulate();
    network.print_history();
}
