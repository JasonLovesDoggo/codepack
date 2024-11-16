use anyhow::{Context, Result};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::{read_to_string, File}, io::{BufWriter, Write}, path::{Path, PathBuf}, sync::Arc};

pub struct DirectoryProcessor {
    extensions: Arc<Vec<String>>,
    suppress_prompt: bool,
    output: String
}

impl DirectoryProcessor {
    pub fn new(extensions: Vec<String>, suppress_prompt: bool, output: String) -> Self {
        Self {
            extensions: Arc::new(extensions),
            suppress_prompt,
            output
        }
    }

    pub fn should_process_file(&self, path: &Path) -> bool {
        if self.extensions.is_empty() {
            return true;
        }

        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| self.extensions.iter().any(|e| e == ext))
            .unwrap_or(false)
    }

    pub fn process_and_write_file(
        &self, path: &Path, writer: &mut BufWriter<File>, pb: &ProgressBar
    ) -> Result<()> {
        let content = read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        // Write file header and content immediately to avoid high memory usage
        writeln!(writer, "\n--- {} ---", path.display())?;
        writeln!(writer, "{}", content)?;

        // Increment progress bar after each file
        pb.inc(1);

        Ok(())
    }

    pub fn run(&self, directory_path: &Path) -> Result<Vec<PathBuf>> {
        let pb = ProgressBar::new(0); // we will update this later based on the number of files
        pb.set_style(ProgressStyle::default_bar()
            .template("{msg} [{elapsed_precise}] {bar:40.cyan} {percent}%")?
            .progress_chars("=>-"));

        // Use WalkBuilder to respect .gitignore files and avoid large directories
        let walker = WalkBuilder::new(directory_path)
            .standard_filters(true)
            .build();

        // Filter and collect the paths
        let file_paths: Vec<PathBuf> = walker
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file() && self.should_process_file(path)
            })
            .map(|entry| entry.path().to_owned())
            .collect();

        pb.set_length(file_paths.len() as u64); // Set the length of the progress bar

        let output_file = File::create(self.output.clone())?;
        let mut writer = BufWriter::new(output_file);

        // Write the prompt if it's not suppressed
        if !self.suppress_prompt {
            writeln!(writer, "This is a .txt file representing an entire directory's contents.")?;
            writeln!(writer, "Each file is separated by a line with its path.\n")?;
        }

        // Process each file one by one and write it to the output immediately
        for path in file_paths.iter().clone() {
            self.process_and_write_file(&path, &mut writer, &pb)?;
        }

        pb.finish_with_message("Directory processing complete");

        Ok(file_paths)
    }
}
