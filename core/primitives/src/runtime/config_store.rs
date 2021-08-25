use crate::runtime::config::RuntimeConfig;
use crate::types::ProtocolVersion;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::ops::Bound;
use std::sync::Arc;

macro_rules! include_config {
    ($file:expr) => {
        include_bytes!(concat!("../../../../nearcore/res/runtime_configs/", $file))
    };
}

/// Stores pairs of protocol versions for which runtime config was updated and
/// the new runtime config in bytes.
/// Protocol versions are given in increasing order. First one is always 0, so that each version is
/// mapped to some config.
static CONFIGS: [(ProtocolVersion, &[u8]); 2] =
    [(0, include_config!("29.json")), (42, include_config!("42.json"))];

/// Stores runtime config for each protocol version where it was updated.
#[derive(Debug)]
pub struct RuntimeConfigStore {
    /// Maps protocol version to the config.
    store: BTreeMap<ProtocolVersion, Arc<RuntimeConfig>>,
}

impl Default for RuntimeConfigStore {
    /// Constructs test store.
    fn default() -> Self {
        Self {
            store: BTreeMap::from_iter([(0, Arc::new(RuntimeConfig::default()))].iter().cloned()),
        }
    }
}

impl RuntimeConfigStore {
    /// Constructs a new store.
    pub fn new() -> Self {
        Self {
            store: BTreeMap::from_iter(CONFIGS.iter().cloned().map(
                |(protocol_version, config_bytes)| {
                    (protocol_version, Arc::new(serde_json::from_slice(config_bytes).unwrap()))
                },
            )),
        }
    }

    /// Returns a `RuntimeConfig` for the corresponding protocol version.
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
    use crate::serialize::to_base;
    use crate::version::ProtocolFeature::LowerStorageCost;
    use near_primitives_core::hash::hash;

    const GENESIS_PROTOCOL_VERSION: ProtocolVersion = 29;
    const RECEIPTS_DEPTH: u64 = 63;

    fn check_config(protocol_version: ProtocolVersion, config_bytes: &[u8]) {
        assert_eq!(
            RuntimeConfigStore::new().get_config(protocol_version).as_ref(),
            &serde_json::from_slice::<RuntimeConfig>(config_bytes).unwrap()
        );
    }

    #[test]
    fn test_get_config() {
        check_config(0, CONFIGS[0].1);
        check_config(GENESIS_PROTOCOL_VERSION - 1, CONFIGS[0].1);
        check_config(GENESIS_PROTOCOL_VERSION, CONFIGS[0].1);
        // First non-trivial version for which runtime config was updated.
        check_config(LowerStorageCost.protocol_version(), CONFIGS[1].1);
        check_config(ProtocolVersion::MAX, CONFIGS.last().unwrap().1);
    }

    #[test]
    fn test_runtime_config_data() {
        let expected_hashes = vec![
            "9T3VNaNdGTiZZvuWiymSxtPdwWKNoJmqoTAaZ4JkuSoL",
            "E82ThZS7KFjpdKmogbMGPwv8nTztxqgSbuCTPRH73XFh",
        ];
        for (i, (_, config_bytes)) in CONFIGS.iter().enumerate() {
            assert_eq!(to_base(&hash(config_bytes)), expected_hashes[i]);
        }
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
