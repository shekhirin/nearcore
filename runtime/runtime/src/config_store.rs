use include_dir::{include_dir, Dir};
use std::path::Path;

const PROJECT_DIR: Dir = include_dir!(".");
use std::fs;

use std::collections::BTreeMap;
use near_primitives::version::ProtocolVersion;
use crate::config::RuntimeConfig;
use near_primitives::types::Gas;

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
        let paths = fs::read_dir("../../../nearcore/res/runtime_configs").unwrap();
        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }
        Self {store: BTreeMap::default()}
    }
}
