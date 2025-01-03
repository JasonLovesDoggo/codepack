use codepack::DirectoryProcessor;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_integration() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new()?;
    let test_file_path = temp_dir.path().join("test.txt");
    let output_file_path = temp_dir.path().join("output.txt");

    // Create a test file
    fs::write(&test_file_path, "This is a test file.")?;

    // Run the codepack processor
    let processor = DirectoryProcessor::new(
        vec!["txt".to_string()],
        vec![],
        false,
        output_file_path.to_str().unwrap().to_string(),
        vec![],
    );
    let files_processed = processor.run(temp_dir.path())?;

    // Check the number of files processed
    println!("Files processed: {}", files_processed);

    // Check the output file
    let output_content = fs::read_to_string(&output_file_path)?;
    println!("Output content:\n{}", output_content);
    assert!(output_content.contains("This is a test file."));

    Ok(())
}