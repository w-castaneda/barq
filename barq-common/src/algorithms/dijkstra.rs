use std::collections::{HashMap, HashSet};

use anyhow::Result;

use crate::{
    graph::{Edge, Node},
    strategy::{Route, RouteHop, RouteInput, RouteOutput, Strategy},
};

/// A routing strategy that uses Dijkstra's algorithm to find the shortest path
/// from the source to the destination.
///
/// The `Dijkstra` strategy calculates the shortest path by considering the
/// accumulated fees and delays of the channels connecting the nodes. It selects
/// the least cost route by exploring all possible paths from the source to the
/// destination.
pub struct Dijkstra;

impl Dijkstra {
    pub fn new() -> Self {
        Dijkstra
    }
}

impl Default for Dijkstra {
    fn default() -> Self {
        Dijkstra::new()
    }
}

/// Find the node with the minimum distance from the source node
///
/// The function takes a list of nodes, a hashmap of distances, and a set of
/// visited nodes and returns the node with the minimum distance from the source
/// node
fn find_min_distance(
    nodes: Vec<&Node>,
    distances: &HashMap<String, u64>,
    visited: &HashSet<String>,
) -> Result<Node> {
    let mut current_node = nodes[0].clone();
    let mut current_node_distance = &u64::MAX;

    for node in nodes {
        let node_distance = distances.get(&node.id).ok_or_else(|| {
            anyhow::anyhow!("Cannot retrive distance of node with id {}", node.id)
        })?;
        if node_distance < current_node_distance && !visited.contains(&node.id) {
            current_node_distance = node_distance;
            current_node = node.clone();
        }
    }

    Ok(current_node)
}

/// Calculate the fee for a channel given the amount to be sent
fn calculate_fee(amount_msat: u64, edge: &Edge) -> u64 {
    edge.base_fee_millisatoshi + (amount_msat * edge.fee_per_millionth) / 1_000_000
}

impl Strategy for Dijkstra {
    fn can_apply(&self, _input: &RouteInput) -> Result<bool> {
        // TODO: Implement the logic to check if the strategy can be applied to the
        // given input

        Ok(true)
    }

    fn route(&self, input: &RouteInput) -> Result<RouteOutput> {
        let mut visited: HashSet<String> = HashSet::new();
        let mut distance: HashMap<String, u64> = HashMap::new();
        let mut route_paths: HashMap<String, Vec<Route>> = HashMap::new();

        for node in input.graph.get_all_nodes() {
            if node.id == input.src_pubkey {
                distance.insert(node.id.clone(), 0);
            } else {
                // FIXME: Why are we using u64::MAX here?
                distance.insert(node.id.clone(), u64::MAX);
            }
            route_paths.insert(node.id.clone(), Vec::<Route>::new());
        }

        loop {
            let current_node = find_min_distance(input.graph.get_all_nodes(), &distance, &visited)?;
            let current_node_id = current_node.id.clone();

            if visited.len() == input.graph.get_all_nodes().len() {
                break;
            }

            let node = input
                .graph
                .get_node(&(current_node_id.clone()))
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Node {} cannot be retrived from Network Graph",
                        current_node_id
                    )
                })?;

            for channel in &node.channels {
                let edge = input.graph.get_edge(channel).ok_or_else(|| {
                    anyhow::anyhow!("Edge {} cannot be retrived from Network Graph", channel)
                })?;

                let fee = calculate_fee(input.amount_msat, edge);
                let current_node_distance = distance.get(&current_node_id).unwrap_or(&u64::MAX);
                let destination = if edge.node1 == current_node_id {
                    edge.node2.clone()
                } else {
                    edge.node1.clone()
                };
                let next_node_distance = *distance.get(&destination).unwrap_or(&u64::MAX);

                if current_node_distance + fee < next_node_distance {
                    distance.insert(destination.clone(), current_node_distance + fee);
                    let mut route_path = route_paths
                        .entry(current_node_id.clone())
                        .or_default()
                        .clone();
                    let route: Route =
                        Route::new(destination.clone(), edge.id.clone(), edge.delay, fee);
                    route_path.push(route);
                    route_paths.insert(destination.clone(), route_path);
                }
            }

            visited.insert(current_node_id.clone());
        }

        let route_path = route_paths
            .entry(input.dest_pubkey.clone())
            .or_default()
            .clone();
        let mut path: Vec<RouteHop> = Vec::<RouteHop>::new();
        let mut total_amt: u64 = input.amount_msat;
        let mut total_delay: u64 = 9;
        for route in route_path.iter().take(route_path.len() - 1) {
            total_amt += route.fee;
            total_delay += route.delay;
        }

        for route in route_path {
            let hop = RouteHop::new(route.id, route.channel, total_delay, total_amt);
            path.push(hop);
            total_amt -= route.fee;
            total_delay -= route.delay;
        }

        Ok(RouteOutput { path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{graph::NetworkGraph, strategy::Router};

    #[test]
    fn test_dijkstra_routing() {
        let mut graph = NetworkGraph::new();
        graph.add_node(Node::new("A"));
        graph.add_node(Node::new("B"));
        graph.add_node(Node::new("C"));
        graph.add_node(Node::new("D"));

        graph.add_edge(Edge::new("channel1", "A", "B", 200, 6, 1, 10));
        graph.add_edge(Edge::new("channel2", "A", "C", 200, 6, 2, 10));
        graph.add_edge(Edge::new("channel3", "B", "D", 200, 6, 1, 10));
        graph.add_edge(Edge::new("channel4", "C", "D", 200, 6, 1, 10));

        let strategies: Vec<Box<dyn Strategy>> = vec![Box::new(Dijkstra::new())];
        let router = Router::new(strategies);
        let input = RouteInput {
            src_pubkey: "A".to_string(),
            dest_pubkey: "D".to_string(),
            cltv: 9,
            amount_msat: 100,
            graph,
        };
        let output = router.execute(&input).expect("Dijsktra Routing Failed");
        let mut route_path: Vec<RouteHop> = Vec::<RouteHop>::new();
        let hop1 = RouteHop::new("B".to_string(), "channel1".to_string(), 15, 101);
        let hop2 = RouteHop::new("D".to_string(), "channel3".to_string(), 9, 100);
        route_path.push(hop1);
        route_path.push(hop2);
        assert_eq!(output.path, route_path);
    }
}
