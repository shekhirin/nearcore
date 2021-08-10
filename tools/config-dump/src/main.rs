use std::io::Result;
use near_primitives::runtime::config::RuntimeConfig;


fn main() -> Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&RuntimeConfig::default())
            .unwrap()
    );
    Ok(())
}
