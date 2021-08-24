use include_dir::{include_dir, Dir};

use crate::runtime::config::RuntimeConfig;
use crate::types::Gas;
use crate::types::ProtocolVersion;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::ops::Bound;
use std::sync::Arc;

macro_rules! include_config {
    ($file:expr) => {
        include_bytes!(Path::new("../../nearcore/res/runtime_configs/").join($file))
    };
}

static CONFIGS: [(ProtocolVersion, &[u8])] =
    *[(0, include_config!("0.json")), (42, include_config!("42.json"))];

/// Stores runtime config for each protocol version where it was updated.
#[derive(Debug)]
pub struct RuntimeConfigStore {
    /// Maps protocol version to the config with possibly modified `max_gas_burnt_view` limit.
    store: BTreeMap<ProtocolVersion, Arc<RuntimeConfig>>,
}

impl RuntimeConfigStore {
    /// Constructs a new store.
    ///
    /// If `max_gas_burnt_view` is provided, the property in wasm limit
    /// configuration will be adjusted to given value.
    pub fn new() -> Self {
        Self { store: BTreeMap::from_iter(CONFIGS.iter()) }
    }

    /// Returns a `RuntimeConfig` for the corresponding protocol version.
    ///
    /// Note that even if some old version is given as the argument, this may
    /// still return configuration which differs from configuration found in
    /// genesis file by the `max_gas_burnt_view` limit.
    pub fn get_config(&self, protocol_version: ProtocolVersion) -> &Arc<RuntimeConfig> {
        self.store
            .range((Bound::Unbounded, Bound::Included(protocol_version)))
            .next_back()
            .unwrap_or_else(|| {
                panic!("Not found RuntimeConfig for protocol version {}", protocol_version)
            })
            .1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::version::ProtocolFeature::LowerStorageCost;

    const GENESIS_PROTOCOL_VERSION: ProtocolVersion = 29;
    const RECEIPTS_DEPTH: u64 = 63;

    #[test]
    fn test_existing_configs() {
        let store = RuntimeConfigStore::new();
        store.get_config(0);
        store.get_config(GENESIS_PROTOCOL_VERSION - 1);
        store.get_config(GENESIS_PROTOCOL_VERSION);
        store.get_config(LowerStorageCost.protocol_version());
        store.get_config(ProtocolVersion::MAX);
    }

    #[test]
    fn test_max_prepaid_gas() {
        let store = RuntimeConfigStore::new();
        for (protocol_version, config) in store.store.iter() {
            assert!(
                config.wasm_config.limit_config.max_total_prepaid_gas
                    / config.transaction_costs.min_receipt_with_function_call_gas()
                    <= 63,
                "The maximum desired depth of receipts for protocol version {} should be at most {}",
                protocol_version,
                RECEIPTS_DEPTH
            );
        }
    }

    #[test]
    fn test_lower_cost() {
        let store = RuntimeConfigStore::new();
        let base_cfg = store.get_config(GENESIS_PROTOCOL_VERSION);
        let new_cfg = store.get_config(LowerStorageCost.protocol_version());
        assert!(base_cfg.storage_amount_per_byte > new_cfg.storage_amount_per_byte);
    }
}
