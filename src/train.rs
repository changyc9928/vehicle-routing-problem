use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    sync::{Arc, Mutex},
};

use crate::{edge::Edge, node::Node, package::Package};

#[derive(Clone)]
pub struct Train {
    name: String,
    capacity: u64,
    load: u64,
    package: HashMap<String, Arc<Mutex<Package>>>,
    location: Location,
    route: HashMap<String, Option<String>>,
    time: u64,
    history: Vec<History>,
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
            .finish()
    }
}

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

    pub fn find_new_target(&mut self) {
        self.route = HashMap::new();
        match &self.location {
            Location::Edge(_) => return,
            Location::Node(n) => {
                let crit = n.lock().unwrap().get_shortest_path();
                for c in crit {
                    let node = c.destination;
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
                            return;
                        }
                    }
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
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn deliver(&mut self, time: u64) {
        let current_node = match self.location.clone() {
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
            Location::Node(current_node) => {
                self.history.push(History::new(time, self.name.clone()));
                let len = self.history.len();
                let mut package_to_be_unload: Vec<Arc<Mutex<Package>>> = vec![];
                let cur_node_name = current_node.lock().unwrap().get_name();
                let mut reach_critical_node = false;
                if time == 0 {
                    reach_critical_node = true;
                }
                for package in self.package.values() {
                    let end_node_name = package
                        .lock()
                        .unwrap()
                        .get_end_node()
                        .lock()
                        .unwrap()
                        .get_name();
                    if cur_node_name == end_node_name {
                        package_to_be_unload.push(package.clone());
                    }
                }
                let mut drop_package_name = vec![];
                for p in package_to_be_unload {
                    let pkg_name = p.lock().unwrap().get_name();
                    self.package.remove(&pkg_name).unwrap();
                    self.load -= p.lock().unwrap().get_weight();
                    current_node.lock().unwrap().add_pick_up_package(p.clone());
                    p.lock().unwrap().arrive();
                    drop_package_name.push(pkg_name);
                    reach_critical_node = true;
                }
                if len >= 2 {
                    self.history[len - 2]
                        .register_arrival(cur_node_name.clone(), drop_package_name);
                }
                let packages = current_node.lock().unwrap().get_package();
                let mut new_package_name = vec![];
                for p in packages {
                    let weight = p.lock().unwrap().get_weight();
                    if self.load + weight <= self.capacity && !p.lock().unwrap().get_arrived() {
                        self.package.insert(p.lock().unwrap().get_name(), p.clone());
                        current_node
                            .lock()
                            .unwrap()
                            .remove_package(p.lock().unwrap().get_name());
                        self.load += weight;
                        new_package_name.push(p.lock().unwrap().get_name());
                    }
                    reach_critical_node = true;
                }
                self.history[len - 1].register_departure(cur_node_name, new_package_name);
                if reach_critical_node {
                    self.find_new_target();
                }
                current_node.clone()
            }
        };
        let curr_name = current_node.lock().unwrap().get_name();
        let next_node = self.route.get(&curr_name);
        if let Some(node) = next_node {
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
            self.location = Location::Edge(
                current_node
                    .lock()
                    .unwrap()
                    .get_edge(node.as_ref().unwrap().to_string()),
            );
        } else {
            self.history.remove(self.history.len() - 1);
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
