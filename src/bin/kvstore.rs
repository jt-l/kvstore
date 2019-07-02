extern crate clap;

use clap::{App, Arg, SubCommand};

use std::process;

use kvstore::KvStore;

fn main() {
    let kvstore = KvStore::new();

    let matches = App::new("kvstore")
        .version("0.1.0")
        .author("James Leahy. <jamesleahy314@gmail.com>")
        .about("An in memory key value store")
        .subcommand(
            SubCommand::with_name("get")
                .about("Get the value of a key from the store")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .required(true)
                        .help("The name of the key"),
                ),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set a value for a given key")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .required(true)
                        .help("The name of the key"),
                )
                .arg(
                    Arg::with_name("value")
                        .index(2)
                        .required(true)
                        .help("The value you want to set"),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove a key and its value from the store")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .required(true)
                        .help("The key that you want to remove"),
                ),
        )
        .get_matches();

    // match the command
    if let Some(matches) = matches.subcommand_matches("get") {
        if let Some(key) = matches.value_of("key") {
            kvstore.get(key.to_string());
        }
    } else if let Some(matches) = matches.subcommand_matches("rm") {
        if let Some(key) = matches.value_of("key") {
            kvstore.get(key.to_string());
        }
    } else if let Some(matches) = matches.subcommand_matches("set") {
        if let Some(key) = matches.value_of("key") {
            if let Some(val) = matches.value_of("value") {
                kvstore.set(key.to_string(), val.to_string());
            }
        }
    }
}
