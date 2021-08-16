use include_dir::{include_dir, Dir};
use std::path::Path;

use std::fs;

use crate::config::RuntimeConfig;
use near_primitives::types::Gas;
use near_primitives::version::ProtocolVersion;
use std::collections::BTreeMap;
use std::iter::FromIterator;

pub struct RuntimeConfigStore {
    /// The runtime configuration taken from the genesis file but with possibly
    /// modified `max_gas_burnt_view` limit.
    store: BTreeMap<ProtocolVersion, RuntimeConfig>,
}

impl RuntimeConfigStore {
    /// Constructs a new object from specified genesis runtime config.
    ///
    /// If `max_gas_burnt_view` is provided, the property in wasm limit
    /// configuration will be adjusted to given value.
    pub fn new(max_gas_burnt_view: Option<Gas>) -> Self {
        let runtime_configs_dir: Dir = include_dir!("../../nearcore/res/runtime_configs");
        Self {
            store: BTreeMap::from_iter(runtime_configs_dir.files().iter().map(|file| {
                let mut config = serde_json::from_slice(file.contents()).unwrap();
                if let Some(gas) = max_gas_burnt_view {
                    config.wasm_config.limit_config.max_gas_burnt_view = gas;
                }
                (
                    file.path().file_stem().unwrap().to_str().unwrap().parse().unwrap(),
                    config,
                )
            })),
        }
    }
}
