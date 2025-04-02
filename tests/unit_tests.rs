use codepack::DirectoryProcessor;
use std::path::Path;

#[test]
fn test_should_process_file_empty_path() {
    let processor = DirectoryProcessor::new(vec![], vec![], false, String::new(), false, vec![]);
    let path = Path::new("");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_regular_file() {
    let processor = DirectoryProcessor::new(vec![], vec![], false, String::new(), false, vec![]);
    let path = Path::new("test.txt");
    assert!(processor.should_process_file(path));
}

#[test]
fn test_should_process_file_included_extension() {
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
fn test_should_process_file_excluded_file() {
    let processor = DirectoryProcessor::new(
        vec![],
        vec!["main.rs".to_string()],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("main.rs");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_excluded_file_by_name() {
    let processor = DirectoryProcessor::new(
        vec![],
        vec!["status".to_string()],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("/home/json/code/src/status");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_should_process_file_excluded_globed_file() {
    let processor = DirectoryProcessor::new(
        vec![],
        vec!["*.py".to_string()],
        false,
        String::new(),
        false,
        vec![],
    );
    let path = Path::new("script.py");
    assert!(!processor.should_process_file(path));
}

#[test]
fn test_validate_output_file() {
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
    let test_file_path = temp_dir.path().join("test.txt");
    std::fs::write(&test_file_path, "This is a test file.").expect("Failed to write file");

    let output_path = test_file_path.to_str().unwrap().to_string();
    let mut mock_input = std::io::Cursor::new(b"n\n");
    let mut mock_output = Vec::new();

    let processor = DirectoryProcessor::new(
        vec!["txt".to_string()],
        vec![],
        false,
        output_path.clone(),
        false,
        vec![],
    );

    // if user chooses not to overwrite, should return false
    assert!(!processor
        .validate_output_file(
            output_path.clone(),
            false,
            &mut mock_input,
            &mut mock_output
        )
        .expect("Failed to validate output file"));

    // if forced overwrite, should return true
    assert!(processor
        .validate_output_file(output_path, true, &mut mock_input, &mut mock_output)
        .expect("Failed to validate output file"));
}

#[test]
fn test_run_and_validate_output_file() {
    let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
    let output_path = temp_dir
        .path()
        .join("test.txt")
        .to_str()
        .unwrap()
        .to_string();

    let processor = DirectoryProcessor::new(
        vec!["txt".to_string()],
        vec![],
        false,
        output_path.clone(),
        false,
        vec![],
    );

    processor
        .run(temp_dir.path())
        .expect("Failed to run processor");

    let mut mock_input = std::io::Cursor::new(b"n\n");
    let mut mock_output = Vec::new();

    // should return true because it is processor generated content
    assert!(processor
        .validate_output_file(output_path, false, &mut mock_input, &mut mock_output)
        .expect("Failed to validate output file"));
}
