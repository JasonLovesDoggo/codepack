use codepack::DirectoryProcessor;
use std::path::Path;

#[test]
fn test_should_process_file_empty_path() {
    let processor = DirectoryProcessor::new(vec![], vec![], false, String::new(), vec![]);
    let path = Path::new("");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_regular_file() {
    let processor = DirectoryProcessor::new(vec![], vec![], false, String::new(), vec![]);
    let path = Path::new("test.txt");
    assert!(processor.should_process_file(path));
}

#[test]
fn test_should_process_file_included_extension() {
    let processor = DirectoryProcessor::new(vec!["rs".to_string()], vec![], false, String::new(), vec![]);
    let path = Path::new("main.rs");
    assert!(processor.should_process_file(path));
}
#[test]
fn test_should_process_file_excluded_file() {
    let processor =
        DirectoryProcessor::new(vec![], vec!["main.rs".to_string()], false, String::new(), vec![]);
    let path = Path::new("main.rs");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_excluded_file_by_name() {
    let processor =
        DirectoryProcessor::new(vec![], vec!["status".to_string()], false, String::new(), vec![]);
    let path = Path::new("/home/json/code/src/status");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_excluded_globed_file() {
    let processor = DirectoryProcessor::new(vec![], vec!["*.py".to_string()], false, String::new(), vec![]);
    let path = Path::new("script.py");
    assert!(!processor.should_process_file(path));
}
