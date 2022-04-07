use pretty_assertions::assert_eq;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use twitter_v2::data::{Expansions, Space};
use twitter_v2::Tweet;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Example<T> {
    data: T,
    includes: Option<Expansions>,
}

fn get_examples(path: impl AsRef<Path>) -> impl Iterator<Item = (PathBuf, serde_json::Value)> {
    fs::read_dir(path.as_ref())
        .unwrap_or_else(|e| panic!("could not open '{}': {}", path.as_ref().display(), e))
        .map(|entry| entry.expect("invalid directory entry"))
        .map(|entry| {
            (
                entry.path(),
                serde_json::from_reader(
                    File::open(entry.path()).expect("could not open file entry"),
                )
                .unwrap_or_else(|e| {
                    panic!("could not read json {}: {}", entry.path().display(), e)
                }),
            )
        })
}

#[test]
fn tweet_serde() {
    for (path, example) in get_examples("./fixtures/data/tweet") {
        let decoded: Example<Vec<Tweet>> = serde_json::from_value(example.clone())
            .unwrap_or_else(|e| panic!("Could not read example '{}': {}", path.display(), e));
        #[cfg(feature = "arbitrary_precision")]
        assert_eq!(
            serde_json::to_value(&decoded).unwrap(),
            example,
            "{}",
            path.display()
        );
    }
}

#[test]
fn space_serde() {
    for (path, example) in get_examples("./fixtures/data/space") {
        let decoded: Example<Vec<Space>> = serde_json::from_value(example.clone())
            .unwrap_or_else(|e| panic!("Could not read example '{}': {}", path.display(), e));
        #[cfg(feature = "arbitrary_precision")]
        assert_eq!(
            serde_json::to_value(&decoded).unwrap(),
            example,
            "{}",
            path.display()
        );
    }
}
