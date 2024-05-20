use serde::{Deserialize, Serialize};

pub trait Strategy {
    fn route(&self, input: &RouteInput) -> RouteOutput;
}

#[derive(Serialize, Deserialize)]
pub struct RouteInput {
    // Define the input structure
    pub source: String,
    pub destination: String,
    pub amount: u64,
}

#[derive(Serialize, Deserialize)]
pub struct RouteOutput {
    // Define the output structure
    pub path: Vec<String>,
    pub total_fees: u64,
}

pub struct Router {
    strategy: Box<dyn Strategy>,
}

impl Router {
    pub fn new(strategy: Box<dyn Strategy>) -> Self {
        Router { strategy }
    }

    pub fn execute(&self, input: &RouteInput) -> RouteOutput {
        self.strategy.route(input)
    }
}
