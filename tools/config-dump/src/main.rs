use std::collections::HashSet;
use std::io::Result;
use std::iter::FromIterator;
use std::path::Path;

use clap::{App, Arg};

fn main() -> Result<()> {
    println!(
        "{}",
        serde_json::to_string(RuntimeConfig::default())
            .unwrap()
    );
    Ok(())
}
