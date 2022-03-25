use std::fs::{self, File};
use std::path::{Path, PathBuf};
use twitter_v2::{ApiResponse, Tweet};

fn get_examples(path: impl AsRef<Path>) -> impl Iterator<Item = (PathBuf, File)> {
    fs::read_dir(path.as_ref())
        .unwrap_or_else(|e| panic!("could not open '{}': {}", path.as_ref().display(), e))
        .map(|entry| entry.expect("invalid directory entry"))
        .map(|entry| {
            (
                entry.path(),
                File::open(entry.path()).expect("could not open file entry"),
            )
        })
}

#[test]
fn tweet_serde() {
    for (path, example) in get_examples("./fixtures/data/tweet") {
        let _ = serde_json::from_reader::<_, ApiResponse<Vec<Tweet>, Option<()>>>(example)
            .unwrap_or_else(|e| panic!("Could not read example '{}': {}", path.display(), e));
    }
}