use nearcore::config::MAINNET_GENESIS_JSON;
use node_runtime::config::RuntimeConfig;
use near_primitives::runtime::config_store::RuntimeConfigStore;
use near_primitives::runtime::config::ActualRuntimeConfig;

#[test]
fn test_compatibility() {
    let default_config = RuntimeConfig::default();
    let actual_runtime_config = ActualRuntimeConfig::new(default_config, None);
    let store = RuntimeConfigStore::new(None);
    for protocol_version in [29u32, 34u32, 42u32, 50u32].iter() {
        let old_config = actual_runtime_config.for_protocol_version(protocol_version.clone());
        let new_config = store.get_config(protocol_version.clone());
        assert_eq!(old_config, new_config);
    }
}
