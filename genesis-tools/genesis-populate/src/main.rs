use std::path::Path;

use clap::{App, Arg};

use nearcore::get_default_home;

use genesis_populate::prepare_and_dump_state;

fn main() {
    let default_home = get_default_home();
    let matches = App::new("Genesis populator")
        .arg(
            Arg::with_name("home")
                .long("home")
                .default_value(&default_home)
                .help("Directory for config and data (default \"~/.near\")")
                .takes_value(true),
        )
        .arg(Arg::with_name("additional-accounts-num").long("additional-accounts-num").required(true).takes_value(true).help("Number of additional accounts per shard to add directly to the trie (TESTING ONLY)"))
        .get_matches();

    let home_dir = matches.value_of("home").map(|dir| Path::new(dir)).unwrap();
    let additional_accounts_num = matches
        .value_of("additional-accounts-num")
        .map(|x| x.parse::<u64>().expect("Failed to parse number of additional accounts."))
        .unwrap();
    prepare_and_dump_state(home_dir, additional_accounts_num);
}
