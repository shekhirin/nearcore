use near_primitives::runtime::config::RuntimeConfig;
use node_runtime::config_store::RuntimeConfigStore;
use std::io::Result;

fn main() -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&RuntimeConfig::default())
            .unwrap()
    );
    let store = RuntimeConfigStore::new(Some(0u64));
    println!(
        "{:?}",
        store
    );
    Ok(())
}
