use std::io::Result;
use near_primitives::runtime::config::RuntimeConfig;
use node_runtime::config_store::RuntimeConfigStore;

fn main() -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&RuntimeConfig::default())
            .unwrap()
    );
    RuntimeConfigStore::new(Some(0u64));
    Ok(())
}
