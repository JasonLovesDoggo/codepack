use anyhow::{Result};
use clap::{Parser};
use prettytable::{Table, Row, Cell};
use std::{path::{Path}};
use codepack::DirectoryProcessor;

#[derive(Parser, Debug)]
#[command(name = "codepack", version, about)]
#[command(about = "Convert local directory contents into a single text file, useful for processing by an LLM.")]
struct Args {
    /// Path to the local directory (first argument)
    directory_path: String,

    /// Output file path (optional)author
    #[arg(short, long)]
    output: Option<String>,

    /// File extensions to include (e.g., -e rs -e toml)
    #[arg(short = 'e', long = "extension")]
    extensions: Vec<String>,

    /// Files to exclude from the output, by name/pattern (e.g. -x *.lock -x LICENSE#[arg(short = 'x', long)]
    excluded_files: Vec<String>,

    /// Suppress the output prompt (description of file formatting)
    #[arg(long)]
    suppress_prompt: bool,
}


fn main() -> Result<()> {
    let mut args = Args::parse();
    let directory_path = Path::new(&args.directory_path);
    
    if !args.output.is_some() {
        args.output = Some({
            let directory_name = directory_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("directory");

            // Use the number of files processed to build the description
            format!("{}_code_pack.txt", directory_name)
        });
    }

    let processor = DirectoryProcessor::new(args.extensions, args.excluded_files, args.suppress_prompt, args.output.clone().unwrap());


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
        Cell::new(&files.to_string()),
    ]));
    table.printstd();

   

    // Print output path
    println!("\nOutput written to: {:?}", args.output.unwrap());

    Ok(())
}
