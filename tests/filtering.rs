use codepack::{DirectoryProcessor, Filter};
use std::path::Path;

#[test]
fn test_should_process_file_empty_path() {
    let processor = DirectoryProcessor::new(
        vec!["rs".to_string()],
        vec!["excluded.rs".to_string()],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_excluded_by_name() {
    let processor = DirectoryProcessor::new(
        vec![],
        vec!["test.lock".to_string()],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("test.lock");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_excluded_by_extension() {
    let processor = DirectoryProcessor::new(
        vec!["rs".to_string()],
        vec![],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("main.py");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_included_by_extension() {
    let processor = DirectoryProcessor::new(
        vec!["rs".to_string()],
        vec![],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("main.rs");
    assert!(processor.should_process_file(path));
}

#[test]
fn test_should_process_file_filter_file_name() {
    let processor = DirectoryProcessor::new(
        vec![],
        vec![],
        false,
        String::new(),
        false,
        vec![Filter::FileName("main.rs".to_string())],
    );
    let path = Path::new("main.rs");
    assert!(processor.should_process_file(path));

    let invalid_path = Path::new("lib.rs");
    assert!(!processor.should_process_file(invalid_path));
}

#[test]
fn test_should_process_file_filter_path_contains() {
    let processor = DirectoryProcessor::new(
        vec![],
        vec![],
        false,
        String::new(),
        false,
        vec![Filter::PathContains("src".to_string())],
    );
    let path = Path::new("src/main.rs");
    assert!(processor.should_process_file(path));

    let invalid_path = Path::new("lib/main.rs");
    assert!(!processor.should_process_file(invalid_path));
}

#[test]
fn test_should_process_file_combined_filters() {
    let processor = DirectoryProcessor::new(
        vec!["rs".to_string()],
        vec![],
        false,
        String::new(),
        false,
        vec![
            Filter::FileName("main".to_string()),
            Filter::PathContains("src".to_string()),
        ],
    );
    let path = Path::new("src/main.rs");
    assert!(processor.should_process_file(path));

    let invalid_path = Path::new("awesome/lib.rs"); // Doesn't match FileName filter
    assert!(!processor.should_process_file(invalid_path));

    let invalid_extension_path = Path::new("src/main.py"); // Doesn't match extension
    assert!(!processor.should_process_file(invalid_extension_path));
}

#[test]
fn test_should_process_file_exclusion_and_filters() {
    let processor = DirectoryProcessor::new(
        vec!["rs".to_string()],
        vec!["excluded.rs".to_string()],
        false,
        String::new(),
        false,
        vec![Filter::PathContains("src".to_string())],
    );
    let excluded_path = Path::new("src/excluded.rs");
    assert!(!processor.should_process_file(excluded_path));

    let valid_path = Path::new("src/main.rs");
    assert!(processor.should_process_file(valid_path));
}
