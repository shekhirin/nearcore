use near_chain_configs::Genesis;
use near_primitives::runtime::config::ActualRuntimeConfig;
use near_primitives::runtime::config_store::RuntimeConfigStore;
use nearcore::config::mainnet_genesis;
use node_runtime::config::RuntimeConfig;

#[test]
fn test_mainnet_compatibility() {
    let genesis = mainnet_genesis();
    let genesis_runtime_config = genesis.config.runtime_config;
    let actual_runtime_config = ActualRuntimeConfig::new(genesis_runtime_config, None);

    let store = RuntimeConfigStore::new(None);
    for protocol_version in [29u32, 34u32, 42u32, 50u32].iter() {
        let old_config = actual_runtime_config.for_protocol_version(protocol_version.clone());
        let new_config = store.get_config(protocol_version.clone());
        assert_eq!(old_config, new_config);
    }
}
