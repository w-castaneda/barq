pub mod direct;

use super::strategy::Strategy;

pub fn get_algorithm(name: &str) -> Option<Box<dyn Strategy>> {
    match name {
        "direct" => Some(Box::new(direct::Direct::new())),
        _ => None,
    }
}
