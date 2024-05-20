use crate::strategy::{RouteInput, RouteOutput, Strategy};

pub struct Direct;

impl Direct {
    pub fn new() -> Self {
        Direct
    }
}

impl Strategy for Direct {
    fn route(&self, input: &RouteInput) -> RouteOutput {
        // Implement the routing logic
        RouteOutput {
            path: vec![input.source.clone(), input.destination.clone()],
            total_fees: 0,
        }
    }
}
