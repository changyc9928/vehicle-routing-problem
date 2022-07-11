use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::node::Node;

pub struct Package {
    name: String,
    weight: u64,
    start: Arc<Mutex<Node>>,
    end: Arc<Mutex<Node>>,
    arrived: bool,
}

impl Debug for Package {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let start_node = self.start.lock().unwrap().get_name();
        let end_node = self.end.lock().unwrap().get_name();
        f.debug_struct(&self.name)
            .field("Weight", &self.weight)
            .field("Start", &start_node)
            .field("Destination", &end_node)
            .finish()
    }
}

impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Package {
    pub fn new(name: String, weight: u64, start: Arc<Mutex<Node>>, end: Arc<Mutex<Node>>) -> Self {
        Self {
            name,
            weight,
            start,
            end,
            arrived: false,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_start_node(&self) -> Arc<Mutex<Node>> {
        self.start.clone()
    }

    pub fn get_end_node(&self) -> Arc<Mutex<Node>> {
        self.end.clone()
    }

    pub fn get_weight(&self) -> u64 {
        self.weight
    }

    pub fn get_arrived(&self) -> bool {
        self.arrived
    }

    pub fn arrive(&mut self) {
        self.arrived = true;
    }
}
