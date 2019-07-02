use assert_cmd::prelude::*;
use kvstore::KvStore;
use predicates::str::contains;
use std::process::Command;

// `kvstore` with no args should exit with a non-zero code.
#[test]
fn cli_no_args() {
    Command::cargo_bin("kvstore").unwrap().assert().failure();
}

// `kvstore -V` should print the version
#[test]
fn cli_version() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["-V"])
        .assert()
        .stdout(contains(env!("CARGO_PKG_VERSION")));
}

// `kvstore get <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_get() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["get", "key1"])
        .assert()
        .failure()
        .stderr(contains("not yet implemented"));
}

// `kvstore set <KEY> <VALUE>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_set() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["set", "key1", "value1"])
        .assert()
        .failure()
        .stderr(contains("not yet implemented"));
}

// `kvstore rm <KEY>` should print "unimplemented" to stderr and exit with non-zero code
#[test]
fn cli_rm() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["rm", "key1"])
        .assert()
        .failure()
        .stderr(contains("not yet implemented"));
}

#[test]
fn cli_invalid_get() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["get"])
        .assert()
        .failure();

    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["get", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_set() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["set"])
        .assert()
        .failure();

    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["set", "missing_field"])
        .assert()
        .failure();

    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["set", "extra", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_rm() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["rm"])
        .assert()
        .failure();

    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["rm", "extra", "field"])
        .assert()
        .failure();
}

#[test]
fn cli_invalid_subcommand() {
    Command::cargo_bin("kvstore")
        .unwrap()
        .args(&["unknown", "subcommand"])
        .assert()
        .failure();
}

// Should get previously stored value
#[test]
fn get_stored_value() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    store.set("key2".to_owned(), "value2".to_owned());

    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));
    assert_eq!(store.get("key2".to_owned()), Some("value2".to_owned()));
}

// Should overwrite existent value
#[test]
fn overwrite_value() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key1".to_owned()), Some("value1".to_owned()));

    store.set("key1".to_owned(), "value2".to_owned());
    assert_eq!(store.get("key1".to_owned()), Some("value2".to_owned()));
}

// Should get `None` when getting a non-existent key
#[test]
fn get_non_existent_value() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    assert_eq!(store.get("key2".to_owned()), None);
}

#[test]
fn remove_key() {
    let mut store = KvStore::new();

    store.set("key1".to_owned(), "value1".to_owned());
    store.remove("key1".to_owned());
    assert_eq!(store.get("key1".to_owned()), None);
}
