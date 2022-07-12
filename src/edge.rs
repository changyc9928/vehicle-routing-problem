use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use crate::node::Node;

/// The edge structure which connects 2 nodes
#[derive(Clone)]
pub struct Edge {
    name: String,
    node1: Arc<Mutex<Node>>,
    node2: Arc<Mutex<Node>>,
    journey_time: i64,
}

impl Debug for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!(
            "{} -> {}",
            self.node1.lock().unwrap().get_name(),
            self.node2.lock().unwrap().get_name()
        ))
        .finish()
    }
}

impl Edge {
    pub fn new(name: String, n1: Arc<Mutex<Node>>, n2: Arc<Mutex<Node>>, jt: i64) -> Self {
        Self {
            name,
            node1: n1,
            node2: n2,
            journey_time: jt,
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_start_node(&self) -> Arc<Mutex<Node>> {
        self.node1.clone()
    }

    pub fn get_end_node(&self) -> Arc<Mutex<Node>> {
        self.node2.clone()
    }

    pub fn get_journey_time(&self) -> i64 {
        self.journey_time
    }
}
