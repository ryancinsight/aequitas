//! The workspace forbids `unsafe_code`; this crate cannot.
//!
//! `#[pyclass]` and `#[pymodule]` expand to `unsafe impl` blocks, and a lint
//! level cannot tell macro-generated code from hand-written code, so the
//! `forbid` the law crate carries is unavailable here. This scan restores the
//! part of that guarantee that matters: no `unsafe` written by hand.

use std::fs;
use std::path::Path;

/// Every `.rs` file under `src/`, recursively.
fn sources(directory: &Path, found: &mut Vec<(String, String)>) {
    let entries = fs::read_dir(directory).expect("the source tree is readable");
    for entry in entries {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            sources(&path, found);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            let text = fs::read_to_string(&path).expect("a readable source file");
            found.push((path.display().to_string(), text));
        }
    }
}

#[test]
fn no_source_file_writes_unsafe() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    sources(&root, &mut files);

    assert!(
        files.len() >= 10,
        "the scan found only {} files, so it is not reaching the tree",
        files.len()
    );

    let mut offenders = Vec::new();
    for (path, text) in &files {
        for (number, line) in text.lines().enumerate() {
            let code = line.split("//").next().unwrap_or(line);
            // Word-boundary match: `unsafe` inside an identifier or a doc
            // comment is not an unsafe block.
            let writes_unsafe = code
                .split(|character: char| !character.is_alphanumeric() && character != '_')
                .any(|word| word == "unsafe");
            if writes_unsafe {
                offenders.push(format!("{path}:{}: {}", number + 1, line.trim()));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "hand-written `unsafe` in a crate that documents having none:\n{}",
        offenders.join("\n")
    );
}
