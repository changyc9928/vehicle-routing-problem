use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
};

use crate::{edge::Edge, node::Node, package::Package};

/// A train structure
#[derive(Clone)]
pub struct Train {
    name: String,
    /// Maximum load of the train
    capacity: u64,
    /// The current load of the train
    load: u64,
    /// Packages carried by the train
    package: HashMap<String, Arc<Mutex<Package>>>,
    /// Current location of the train
    location: Location,
    /// A dictionary to navigate the train to next critical node
    route: HashMap<String, Option<String>>,
    /// Time travelled on an edge
    time: u64,
    /// The travel history of the train
    history: Vec<History>,
    /// Whether the train has no package to deliver
    end_trip: bool,
}

impl Debug for Train {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&self.name)
            .field("Capacity", &self.capacity)
            .field("Load", &self.load)
            .field("Package", &self.package.keys())
            .field("Location", &self.location)
            .field("Time travelled", &self.time)
            .field("End", &self.end_trip)
            .finish()
    }
}

/// Location enum as a train can either be on an edge or a node
#[derive(Clone)]
pub enum Location {
    Edge(Arc<Mutex<Edge>>),
    Node(Arc<Mutex<Node>>),
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Edge(e) => f.debug_struct(&e.lock().unwrap().get_name()).finish(),
            Self::Node(n) => f.debug_struct(&n.lock().unwrap().get_name()).finish(),
        }
    }
}

impl Train {
    pub fn new(name: String, capacity: u64, location: Arc<Mutex<Node>>) -> Self {
        Self {
            name,
            capacity,
            load: 0,
            package: HashMap::new(),
            location: Location::Node(location),
            route: HashMap::new(),
            time: 0,
            history: vec![],
            end_trip: false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    /// Function to get a new target for the train if it has already reached a critical node
    pub fn find_new_target(&mut self) {
        self.route = HashMap::new();
        match &self.location {
            Location::Edge(_) => return,
            Location::Node(n) => {
                // Loop for all critical node, this vector is sorted
                let crit = n.lock().unwrap().get_shortest_path();
                // Start from the nearest critical node
                for c in crit {
                    let node = c.destination;
                    // See whether there is any package on the train can be delivered to there
                    for p in node.lock().unwrap().get_drop_off() {
                        if self.package.contains_key(&p) {
                            // let start_name = n.lock().unwrap().get_name();
                            // return Some(
                            //     c.path
                            //         .get(&start_name)
                            //         .unwrap()
                            //         .as_ref()
                            //         .unwrap()
                            //         .to_string(),
                            // );
                            self.route = c.path.clone();
                            return; // Early end the loop if found
                        }
                    }
                    // If no, see if there is any package can be picked up there
                    for p in node.lock().unwrap().get_package() {
                        let p_weight = p.lock().unwrap().get_weight();
                        if !p.lock().unwrap().get_arrived() && self.load + p_weight <= self.capacity
                        {
                            // let start_name = n.lock().unwrap().get_name();
                            // return Some(
                            //     c.path
                            //         .get(&start_name)
                            //         .unwrap()
                            //         .as_ref()
                            //         .unwrap()
                            //         .to_string(),
                            // );
                            self.route = c.path.clone();
                            // self.load += p_weight;
                            return; // Early end the loop if found
                        }
                    }
                }
            }
        }
    }

    /// Function to move the train
    pub fn deliver(&mut self, time: u64) {
        let current_node = match self.location.clone() {
            // If it is on an edge, just increment its travel time
            // If it is reaching a node, change its location to the node
            // and call this function again to drop/pick up package
            Location::Edge(edge) => {
                if self.time < edge.lock().unwrap().get_journey_time() as u64 {
                    self.time += 1;
                    return;
                }
                self.time = 0;
                let node = edge.lock().unwrap().get_end_node();
                self.location = Location::Node(node);
                self.deliver(time);
                return;
            }
            // If the it is on a node
            Location::Node(current_node) => {
                // Create a new history as described in the assignment
                self.history.push(History::new(time, self.name.clone()));
                let len = self.history.len();
                // Get all the packages to be unloaded
                let mut package_to_be_unload: Vec<Arc<Mutex<Package>>> = vec![];
                let cur_node_name = current_node.lock().unwrap().get_name();
                let reach_critical_node = current_node.lock().unwrap().is_critical();
                // For all package on the train
                for package in self.package.values() {
                    let end_node_name = package
                        .lock()
                        .unwrap()
                        .get_end_node()
                        .lock()
                        .unwrap()
                        .get_name();
                    // Compare the destination of the package with current node
                    if cur_node_name == end_node_name {
                        package_to_be_unload.push(package.clone());
                    }
                }
                // Get all the name of the packages to be unloaded
                let mut drop_package_name = vec![];
                for p in package_to_be_unload {
                    let pkg_name = p.lock().unwrap().get_name();
                    // Unload the package from the train
                    self.package.remove(&pkg_name).unwrap();
                    // Decrement the load
                    self.load -= p.lock().unwrap().get_weight();
                    // Add this package to current node
                    current_node.lock().unwrap().add_pick_up_package(p.clone());
                    // Mark this package as delivered
                    p.lock().unwrap().arrive();
                    drop_package_name.push(pkg_name);
                }
                // Update this drop off node to the last history record
                if len >= 2 {
                    self.history[len - 2]
                        .register_arrival(cur_node_name.clone(), drop_package_name);
                }
                // Get all the packages on this node
                let packages = current_node.lock().unwrap().get_package();
                let mut new_package_name = vec![];
                for p in packages {
                    let weight = p.lock().unwrap().get_weight();
                    // Check whether this package has been delivered and the train can hold the package
                    if self.load + weight <= self.capacity && !p.lock().unwrap().get_arrived() {
                        // Load the package to the train
                        self.package.insert(p.lock().unwrap().get_name(), p.clone());
                        current_node
                            .lock()
                            .unwrap()
                            .remove_package(p.lock().unwrap().get_name());
                        // Increment the load
                        self.load += weight;
                        new_package_name.push(p.lock().unwrap().get_name());
                    }
                }
                // Update the current history record
                self.history[len - 1].register_departure(cur_node_name, new_package_name);
                if reach_critical_node {
                    // Find the nearest valid critical node from here if current node is a critical node
                    self.find_new_target();
                }
                current_node.clone()
            }
        };
        // Move to the next node based on the dictionary it holds
        let curr_name = current_node.lock().unwrap().get_name();
        let next_node = self.route.get(&curr_name);
        if let Some(Some(node)) = next_node {
            // println!(
            //     "W={}, T={}, N1={}, P1={}, N2={}, P2={}",
            //     time,
            //     self.name,
            //     curr_name,
            //     "",
            //     node.as_ref().unwrap(),
            //     ""
            // );
            self.time += 1;
            self.location = Location::Edge(current_node.lock().unwrap().get_edge(node.to_string()));
        } else {
            // If there is no node to go, delete the current history entry because it has no destination
            self.history.remove(self.history.len() - 1);
            // Mark the train as end of trip
            self.end_trip = true;
        }
    }

    pub fn get_history(&self) -> Vec<History> {
        self.history.clone()
    }

    pub fn is_not_end(&self) -> bool {
        self.end_trip
    }
}

/// A structure used to display the simulation output
#[derive(Clone, Debug)]
pub struct History {
    w: u64,
    t: String,
    n1: String,
    p1: Vec<String>,
    n2: String,
    p2: Vec<String>,
}

impl PartialEq for History {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl Eq for History {}

impl PartialOrd for History {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for History {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.w.cmp(&other.w)
    }
}

impl Display for History {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "W={}, T={}, N1={}, P1={:?}, N2={}, P2={:?}",
            self.w, self.t, self.n1, self.p1, self.n2, self.p2
        ))
    }
}

impl History {
    pub fn new(w: u64, t: String) -> Self {
        Self {
            w,
            t,
            n1: "".to_string(),
            p1: vec![],
            n2: "".to_string(),
            p2: vec![],
        }
    }

    pub fn register_departure(&mut self, n1: String, p1: Vec<String>) {
        self.n1 = n1;
        self.p1 = p1;
    }

    pub fn register_arrival(&mut self, n2: String, p2: Vec<String>) {
        self.n2 = n2;
        self.p2 = p2;
    }
}
