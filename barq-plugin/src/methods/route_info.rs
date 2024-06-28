use serde::{Deserialize, Serialize};
use serde_json as json;
use serde_json::Value;

use clightningrpc_plugin::{error, errors::PluginError, plugin::Plugin};

use barq_common::strategy::{RouteHop, RouteInput};

use crate::{
    methods::{pay::NodeInfo, utils::graph::build_network_graph},
    plugin::State,
};

/// Request payload for Barq route info RPC method
#[derive(Deserialize, Serialize)]
pub struct BarqRouteInfoRequest {
    pub dest_pubkey: String,
    pub amount_msat: u64,
    pub cltv: u64,
}

/// Response payload for Barq route info RPC method
#[derive(Deserialize, Serialize)]
pub struct BarqRouteInfoResponse {
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_info: Option<Vec<RouteHop>>,
}

/// Barq RPC method to get route information
pub fn barq_route_info(plugin: &mut Plugin<State>, request: Value) -> Result<Value, PluginError> {
    log::info!("barqrouteinfo called with request: {}", request);
    let request: BarqRouteInfoRequest = json::from_value(request).map_err(|err| error!("{err}"))?;
    let state = &plugin.state;
    let router = state.router();

    let node_info: NodeInfo = state
        .call("getinfo", serde_json::json!({}))
        .map_err(|err| error!("Error calling CLN RPC method: {err}"))?;

    // Build the network graph from the plugin state
    let network_graph = build_network_graph(state)?;

    let input = RouteInput {
        src_pubkey: node_info.id.clone(),
        dest_pubkey: request.dest_pubkey.clone(),
        amount_msat: request.amount_msat,
        cltv: request.cltv,
        graph: network_graph,
    };

    let output = router.execute(&input);
    let response = match output {
        Ok(output) => BarqRouteInfoResponse {
            status: "success".to_string(),
            message: None,
            route_info: Some(output.path),
        },
        Err(err) => BarqRouteInfoResponse {
            status: "failure".to_string(),
            message: Some(format!("barqrouteinfo execution failed: {}", err)),
            route_info: None,
        },
    };

    Ok(json::to_value(response)?)
}
