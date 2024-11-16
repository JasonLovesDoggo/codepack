use anyhow::{Result};
use clap::{Parser};
use chrono::Local;
use prettytable::{Table, Row, Cell};
use std::{path::{Path, PathBuf}};
use codedump::DirectoryProcessor;

#[derive(Parser, Debug)]
#[command(name = "codedump")]
#[command(about = "Convert local directory contents into a single text file, useful for processing by an LLM.")]
struct Args {
    /// Path to the local directory (first argument)
    directory_path: String,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// File extensions to include (e.g., -e rs -e toml)
    #[arg(short = 'e', long = "extension")]
    extensions: Vec<String>,

    /// Suppress the output prompt (description of file formatting)
    #[arg(long)]
    suppress_prompt: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let directory_path = Path::new(&args.directory_path);

    let processor = DirectoryProcessor::new(args.extensions, args.suppress_prompt);

    // Parse output path if provided
    let output_path = args.output.map(PathBuf::from);

    // Start the timer
    let start_time = std::time::Instant::now();

    // Run the processing
    let files = processor.run(directory_path)?;

    // Calculate elapsed time
    let duration = start_time.elapsed();
    let formatted_time = format!("{:?}", duration);
    

    // Output the stats and details in a pretty table
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Total time taken"),
        Cell::new(&formatted_time),
    ]));
    table.add_row(Row::new(vec![
        Cell::new("Number of files processed"),
        Cell::new(&files.len().to_string()),
    ]));
    table.printstd();

    // Write the output to the file with description of the output
    let output_path = output_path.unwrap_or_else(|| {
        let directory_name = directory_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("directory");

        // Use the number of files processed to build the description
        let file_count = files.len();
        PathBuf::from(format!("{}_{}files.txt", directory_name, file_count))
    });

    // Print output path
    println!("\nOutput written to: {}", output_path.display());

    Ok(())
}
