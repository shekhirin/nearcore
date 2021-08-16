use include_dir::{include_dir, Dir};
use std::path::Path;

use std::fs;

use std::collections::BTreeMap;
use near_primitives::version::ProtocolVersion;
use crate::config::RuntimeConfig;
use near_primitives::types::Gas;
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
        // for file in runtime_configs_dir.files() {
        //     println!("Name: {}", file.path().display());
        //     let protocol_version: ProtocolVersion = file.path().file_stem().unwrap().to_str().unwrap().parse().unwrap();
        //     println!("PV: {}", protocol_version);
        //     let config: RuntimeConfig = serde_json::from_slice(file.contents()).unwrap();
        //     println!("RC: {:?}", config);
        // }
        // let paths = fs::read_dir("../../../nearcore/res/runtime_configs").unwrap();
        // for path in paths {
        //     println!("Name: {}", path.unwrap().path().display())
        // }
        Self {store: BTreeMap::from_iter(
            runtime_configs_dir.files().iter().map(|file| {
                (file.path().file_stem().unwrap().to_str().unwrap().parse().unwrap(),
                 serde_json::from_slice(file.contents()).unwrap()
                )
            })
        )}
    }
}
