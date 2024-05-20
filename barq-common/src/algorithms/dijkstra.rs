use crate::strategy::{RouteInput, RouteOutput, Strategy};

pub struct Dijkstra;

impl Dijkstra {
    pub fn new() -> Self {
        Dijkstra
    }
}

impl Strategy for Dijkstra {
    fn route(&self, input: &RouteInput) -> RouteOutput {
        // Implement the routing logic
        RouteOutput {
            path: vec![input.source.clone(), input.destination.clone()],
            total_fees: 0,
        }
    }
}
