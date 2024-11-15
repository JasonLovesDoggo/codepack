use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use clap::Parser;
use rayon::prelude::*;
use ignore::Walk;
use chrono::Local;
use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Parser, Debug)]
#[command(name = "repo-to-text")]
#[command(about = "Convert local repository contents to a single text file")]
struct Args {
    /// Path to the local repository
    #[arg(short, long)]
    repo_path: String,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,

    /// File extensions to include (e.g., -e rs -e toml)
    #[arg(short = 'e', long = "extension")]
    extensions: Vec<String>,
}

struct RepoProcessor {
    extensions: Arc<Vec<String>>,
}

impl RepoProcessor {
    fn new(extensions: Vec<String>) -> Self {
        Self {
            extensions: Arc::new(extensions),
        }
    }

    fn should_process_file(&self, path: &Path) -> bool {
        if self.extensions.is_empty() {
            return true;
        }

        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.extensions.iter().any(|e| e == ext))
            .unwrap_or(false)
    }

    fn process_repository(&self, repo_path: &Path) -> Result<Vec<(PathBuf, String)>> {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .unwrap(),
        );
        pb.set_message("Processing repository...");

        // Use rayon for parallel processing of files
        let files: Result<Vec<_>> = Walk::new(repo_path)
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
            .filter(|entry| self.should_process_file(entry.path()))
            .par_bridge()
            .map(|entry| {
                let path = entry.path().to_owned();
                let content = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read file: {}", path.display()))?;
                Ok((path, content))
            })
            .collect();

        pb.finish_with_message("Repository processing complete");
        files
    }

    fn write_output(&self, files: Vec<(PathBuf, String)>, output_path: &Path) -> Result<()> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        writeln!(writer, "Repository Export")?;
        writeln!(writer, "Generated at: {}", Local::now().format("%Y-%m-%d %H:%M:%S"))?;
        writeln!(writer, "Number of files: {}\n", files.len())?;

        for (path, content) in files {
            writeln!(writer, "\n--- {} ---", path.display())?;
            writeln!(writer, "{}\n", content)?;
        }

        writer.flush()?;
        Ok(())
    }

    fn run(&self, args: Args) -> Result<()> {
        let repo_path = Path::new(&args.repo_path);
        if !repo_path.is_dir() {
            anyhow::bail!("Repository path does not exist or is not a directory");
        }

        let files = self.process_repository(repo_path)?;

        let output_path = match args.output {
            Some(path) => PathBuf::from(path),
            None => {
                let timestamp = Local::now().format("%Y%m%d%H%M%S");
                let repo_name = repo_path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("repo");
                PathBuf::from(format!("{}_{}.txt", repo_name, timestamp))
            }
        };

        self.write_output(files, &output_path)?;
        println!("Output written to: {}", output_path.display());

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let processor = RepoProcessor::new(args.extensions.clone());
    processor.run(args)
}