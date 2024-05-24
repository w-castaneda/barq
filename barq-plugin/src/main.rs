mod methods;
mod plugin;

use anyhow::Result;

/// Main entry point for Barq
fn main() -> Result<()> {
    let plugin = plugin::build_plugin()?;
    plugin.start();
    Ok(())
}
