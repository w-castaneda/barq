#![allow(unused)]

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

use clightningrpc_gossip_map::GossipMap;
use clightningrpc_plugin::errors::PluginError;

use barq_common::graph::{Channel, NetworkGraph, Node};

use crate::plugin::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct P2PNetworkGraph {
    nodes: HashMap<String, Node>,
    channels: HashMap<String, Channel>,
}

impl Default for P2PNetworkGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl P2PNetworkGraph {
    /// Creates a new, empty network graph.
    pub fn new() -> Self {
        P2PNetworkGraph {
            nodes: HashMap::new(),
            channels: HashMap::new(),
        }
    }

    /// Adds a node to the network graph.
    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    /// Adds a channel to the network graph.
    pub fn add_channel(&mut self, channel: Channel) {
        self.channels.insert(channel.id.clone(), channel.clone());
        self.nodes
            .get_mut(&channel.node1)
            .unwrap()
            .add_channel(&channel);
        self.nodes
            .get_mut(&channel.node2)
            .unwrap()
            .add_channel(&channel);
    }
}

impl NetworkGraph for P2PNetworkGraph {
    fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }

    fn get_channel(&self, id: &str) -> Option<&Channel> {
        self.channels.get(id)
    }

    fn get_nodes(&self) -> Vec<&Node> {
        self.nodes.values().collect()
    }

    fn get_channels(&self) -> Vec<&Channel> {
        self.channels.values().collect()
    }

    fn has_p2p_info(&self) -> bool {
        true
    }
}

/// Function to build the network graph using the plugin state.
pub fn build_p2p_network_graph(
    _state: &State,
    _gossip_map: &GossipMap,
) -> Result<P2PNetworkGraph, PluginError> {
    let graph = P2PNetworkGraph::new();

    // TODO: Use the gossip map to build the network graph

    Ok(graph)
}