use std::collections::{HashMap, HashSet};
use std::fmt::{self, Debug};
use std::hash::Hash;
use std::sync::{Arc, Mutex};

use crate::train::Train;
use crate::{edge::Edge, package::Package};

#[derive(Clone)]
pub struct Node {
    name: String,
    edge: HashMap<String, Arc<Mutex<Edge>>>,
    pick_up_package: HashMap<String, Arc<Mutex<Package>>>,
    drop_off_package: HashSet<String>,
    train_here: HashMap<String, Arc<Mutex<Train>>>,
    shortest_path_to_other_critical_nodes: Vec<ShortestPathToCritical>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Node {}", self.name))
            .field(
                "Accessible node(s) from here",
                &self
                    .edge
                    .clone()
                    .into_iter()
                    .map(|x| {
                        format!(
                            "{}",
                            x.1.lock()
                                .unwrap()
                                .get_end_node()
                                .lock()
                                .unwrap()
                                .get_name()
                        )
                    })
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .field(
                "Path to critical nodes",
                &self.shortest_path_to_other_critical_nodes,
            )
            .field(
                "Pickup point",
                &self
                    .pick_up_package
                    .clone()
                    .into_iter()
                    .map(|x| x.0)
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .field(
                "Drop off point",
                &self
                    .drop_off_package
                    .clone()
                    .into_iter()
                    .collect::<Vec<String>>()
                    .join(", "),
            )
            .finish()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Node {}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Node {
    pub fn new(s: String) -> Self {
        Self {
            name: s,
            edge: HashMap::new(),
            pick_up_package: HashMap::new(),
            drop_off_package: HashSet::new(),
            train_here: HashMap::new(),
            shortest_path_to_other_critical_nodes: vec![],
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_neighbours(&self) -> Vec<(Arc<Mutex<Node>>, i64)> {
        let mut ret = vec![];
        for edge in &self.edge {
            let end_node = edge.1.lock().unwrap().get_end_node();
            ret.push((end_node, edge.1.lock().unwrap().get_journey_time()));
        }
        ret
    }

    pub fn add_edge(&mut self, edge: Arc<Mutex<Edge>>) {
        self.edge
            .insert(edge.lock().unwrap().get_name(), edge.clone());
    }

    pub fn add_pick_up_package(&mut self, package: Arc<Mutex<Package>>) {
        self.pick_up_package
            .insert(package.lock().unwrap().get_name(), package.clone());
    }

    pub fn add_drop_off_package(&mut self, package_name: String) {
        self.drop_off_package.insert(package_name);
    }

    pub fn add_train(&mut self, train: Arc<Mutex<Train>>) {
        self.train_here
            .insert(train.lock().unwrap().get_name(), train.clone());
    }

    pub fn is_critical(&self) -> bool {
        !self.pick_up_package.is_empty()
            || !self.drop_off_package.is_empty()
            || !self.train_here.is_empty()
    }

    pub fn add_shortest_path(
        &mut self,
        dis: i64,
        dest: Arc<Mutex<Node>>,
        path: HashMap<String, Option<String>>,
    ) {
        self.shortest_path_to_other_critical_nodes
            .push(ShortestPathToCritical {
                distance_to_destination: dis,
                destination: dest,
                path,
            });
    }

    pub fn sort_shortest_dest(&mut self) {
        self.shortest_path_to_other_critical_nodes.sort_unstable();
    }

    pub fn get_shortest_path(&self) -> Vec<ShortestPathToCritical> {
        self.shortest_path_to_other_critical_nodes.clone()
    }

    pub fn get_package(&self) -> Vec<Arc<Mutex<Package>>> {
        let mut ret = vec![];
        for p in self.pick_up_package.values() {
            ret.push(p.clone());
        }
        ret
    }

    pub fn remove_package(&mut self, package: String) {
        self.pick_up_package.remove(&package);
    }

    pub fn contain_package(&self, name: String) -> bool {
        self.pick_up_package.contains_key(&name)
    }

    pub fn get_drop_off(&self) -> Vec<String> {
        self.drop_off_package
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
    }

    pub fn get_edge(&self, dest: String) -> Arc<Mutex<Edge>> {
        for e in self.edge.values() {
            if e.lock().unwrap().get_end_node().lock().unwrap().get_name() == dest {
                return e.clone();
            }
        }
        panic!("Edge not found");
    }
}

#[derive(Clone)]
pub struct ShortestPathToCritical {
    pub distance_to_destination: i64,
    pub destination: Arc<Mutex<Node>>,
    pub path: HashMap<String, Option<String>>,
}

impl Debug for ShortestPathToCritical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&self.destination.lock().unwrap().get_name())
            .field("Distance", &self.distance_to_destination)
            .field("Path", &self.path)
            .finish()
    }
}

impl Eq for ShortestPathToCritical {}

impl PartialEq for ShortestPathToCritical {
    fn eq(&self, other: &Self) -> bool {
        self.distance_to_destination == other.distance_to_destination
            && *self.destination.lock().unwrap() == *other.destination.lock().unwrap()
    }
}

impl PartialOrd for ShortestPathToCritical {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ShortestPathToCritical {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance_to_destination
            .cmp(&other.distance_to_destination)
    }
}
