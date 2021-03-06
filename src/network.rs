use std::{
    collections::{HashMap, HashSet},
    sync::{Arc, Mutex},
};

use rudac::heap::FibonacciHeap;

use crate::{
    edge::Edge,
    node::Node,
    package::Package,
    train::{History, Location, Train},
};

/// This is a struct to hold the hashmap to every instance contained in itself
/// Hashmap data structure allows O(1) time of searching for any instance
#[derive(Clone, Debug)]
pub struct Network {
    node: HashMap<String, Arc<Mutex<Node>>>,
    edge: HashMap<String, Arc<Mutex<Edge>>>,
    train: HashMap<String, Arc<Mutex<Train>>>,
    package: HashMap<String, Arc<Mutex<Package>>>,
    /// The overall time simulated in this network instance
    time: u64,
}

impl Network {
    pub fn new() -> Self {
        Self {
            node: HashMap::new(),
            edge: HashMap::new(),
            train: HashMap::new(),
            package: HashMap::new(),
            time: 0,
        }
    }

    /// Function to register new nodes in this network graph
    pub fn initialize_node(&mut self, n: Vec<Node>) {
        let mut nodes = HashMap::new();
        for node in n {
            nodes.insert(node.get_name(), Arc::new(Mutex::new(node)));
        }
        self.node = nodes;
    }

    pub fn get_node(&self, name: String) -> Arc<Mutex<Node>> {
        self.node.get(&name).unwrap().clone()
    }

    /// Function to register new edges in this network graph
    /// The start node of the edges will also registering this edge in it
    pub fn initialize_edge(&mut self, e: Vec<Edge>) {
        let mut edges = HashMap::new();
        for edge in e {
            let new_edge = Arc::new(Mutex::new(edge));
            edges.insert(new_edge.lock().unwrap().get_name(), new_edge.clone());
            let start_node = new_edge.lock().unwrap().get_start_node();
            start_node.lock().unwrap().add_edge(new_edge);
        }
        self.edge = edges;
    }

    pub fn get_edge(&self, name: String) -> Arc<Mutex<Edge>> {
        self.edge.get(&name).unwrap().clone()
    }

    /// Function to register new trains in this network graph
    /// The initial location node of the train will also register the respective train
    pub fn initialize_train(&mut self, t: Vec<Train>) {
        let mut trains = HashMap::new();
        for train in t {
            let loc = train.clone();
            let loc = loc.get_location();
            let new_train = Arc::new(Mutex::new(train));
            trains.insert(new_train.lock().unwrap().get_name(), new_train.clone());
            match loc {
                Location::Node(n) => n.lock().unwrap().add_train(new_train.clone()),
                _ => panic!("Train is not initalized at a node."),
            };
        }
        self.train = trains;
    }

    pub fn get_train(&self, name: String) -> Arc<Mutex<Train>> {
        self.train.get(&name).unwrap().clone()
    }

    /// Function to register new packages in this network graph
    /// The nodes at which it is alocated and will be dropped at will be registered by this package as well
    pub fn initialize_package(&mut self, p: Vec<Package>) {
        let mut packages = HashMap::new();
        for package in p {
            let new_package = Arc::new(Mutex::new(package));
            packages.insert(new_package.lock().unwrap().get_name(), new_package.clone());
            let node = new_package.lock().unwrap().get_start_node();
            node.lock()
                .unwrap()
                .add_pick_up_package(new_package.clone());
            let node = new_package.lock().unwrap().get_end_node();
            node.lock()
                .unwrap()
                .add_drop_off_package(new_package.lock().unwrap().get_name());
        }
        self.package = packages;
    }

    pub fn get_package(&self, name: String) -> Arc<Mutex<Package>> {
        self.package.get(&name).unwrap().clone()
    }

    /// Normal Dijkstra's Shortest Path Algorithm
    /// Implemented using Fibonacci Heap for faster calculation
    pub fn dijkstra(
        &self,
        source: Arc<Mutex<Node>>,
    ) -> (HashMap<String, i64>, HashMap<String, Option<String>>) {
        let mut distance = HashMap::new();
        let mut prev: HashMap<String, Option<String>> = HashMap::new();
        let mut visited = HashSet::new();
        let source_name = source.lock().unwrap().get_name();
        distance.insert(source_name.clone(), 0);

        let mut priority_queue = FibonacciHeap::init_min();
        priority_queue.push(DistanceToSource {
            distance: 0,
            node: source_name.clone(),
        });

        // Initialize all the nodes to infinite distance in the Fib Heap
        for v in self.node.values() {
            let v_name = v.lock().unwrap().get_name();
            if v_name != source_name {
                distance.insert(v_name.clone(), i64::MAX);
                prev.insert(v_name.clone(), None);
            }
            priority_queue.push(DistanceToSource {
                distance: i64::MAX,
                node: v_name,
            });
        }

        while !priority_queue.is_empty() {
            let start_node = self.get_node(priority_queue.pop().unwrap().node);
            let start_node_name = start_node.lock().unwrap().get_name();
            if visited.contains(&start_node_name) {
                continue;
            }

            visited.insert(start_node_name.clone());

            for neighbour in start_node.lock().unwrap().get_neighbours() {
                // The node is unreachable from other part of graph
                if *distance.get(&start_node_name).unwrap() == i64::MAX {
                    continue;
                }
                let new_distance = distance.get(&start_node_name).unwrap() + neighbour.1;
                let neighbour_node_name = neighbour.0.lock().unwrap().get_name();
                if visited.contains(&neighbour_node_name) {
                    continue;
                }

                if new_distance < *distance.get(&neighbour_node_name).unwrap()
                    && *distance.get(&start_node_name).unwrap() != i64::MAX
                {
                    distance.insert(neighbour_node_name.clone(), new_distance);
                    prev.insert(neighbour_node_name.clone(), Some(start_node_name.clone()));
                    // Add the new node with shorter distance
                    // Actually it should be implemented as decreasing its priority but this Fib Heap library doesn't support the operation
                    // So the workaround is to introduce a new visited Set to skip the nodes which are already visited
                    priority_queue.push(DistanceToSource {
                        distance: new_distance,
                        node: neighbour_node_name,
                    });
                }
            }
        }

        // for d in distance.clone().into_iter() {
        //     distance.remove(&d.0);
        // }

        return (distance, prev);
    }

    /// Function to calculate all the shortest path between initial train location, package pick up point and drop off point
    /// Using the Dijkstra's Algorithm above
    pub fn calculate_shortest_distance_between_packages(&self) {
        for p in self.package.values() {
            let source = p.lock().unwrap().get_end_node();
            let source_name = source.lock().unwrap().get_name();

            let (dist, prev) = self.dijkstra(source.clone());

            for (node, distance) in dist.iter() {
                let node = self.get_node(node.to_string());
                let node_name = node.lock().unwrap().get_name();
                if node.lock().unwrap().is_critical() && node_name != source_name {
                    node.lock()
                        .unwrap()
                        .add_shortest_path(*distance, source.clone(), prev.clone());
                }
            }

            let source = p.lock().unwrap().get_start_node();
            let source_name = source.lock().unwrap().get_name();

            let (dist, prev) = self.dijkstra(source.clone());

            for (node, distance) in dist.iter() {
                let node = self.get_node(node.to_string());
                let node_name = node.lock().unwrap().get_name();
                if node.lock().unwrap().is_critical() && node_name != source_name {
                    node.lock()
                        .unwrap()
                        .add_shortest_path(*distance, source.clone(), prev.clone());
                }
            }
        }
        for node in self.node.values() {
            node.lock().unwrap().sort_shortest_dest();
        }
    }

    /// Function to check whether all package has been delivered to the destination
    /// Used to stop the simulation once all of them has been delivered
    fn has_all_package_delivered(&self) -> bool {
        for p in self.package.values() {
            if !p.lock().unwrap().get_arrived() {
                return false;
            }
        }
        return true;
    }

    /// Main function to simulate the project
    /// The simulation is broken into 1 unit of time for each loop
    /// All the train will be finding the shortest critical nodes if it is on another critical node
    pub fn simulate(&mut self) {
        while !self.has_all_package_delivered() {
            let mut all_train_stopped = true;
            for t in self.train.values() {
                let end = t.lock().unwrap().is_not_end();
                if !end {
                    t.lock().unwrap().deliver(self.time);
                    all_train_stopped = false;
                }
            }
            self.time += 1;
            // println!("{:#?}", self);
            if all_train_stopped {
                if !self.has_all_package_delivered() {
                    panic!("Some packages are not reachable");
                }
                break;
            }
        }
    }

    /// Function to output the simulation history as shown in the assignment description
    pub fn print_history(&self) {
        let mut history = self
            .train
            .clone()
            .into_values()
            .flat_map(|x| x.lock().unwrap().get_history())
            .collect::<Vec<History>>();
        history.sort();
        for h in history {
            println!("{}", h);
        }
    }
}

/// A special structure used to hold the distance-node information in the Fib Heap
struct DistanceToSource {
    distance: i64,
    node: String,
}

impl Eq for DistanceToSource {}

impl Ord for DistanceToSource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialEq for DistanceToSource {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for DistanceToSource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
