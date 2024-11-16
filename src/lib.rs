mod constants;

use anyhow::Result;
use globset::{GlobBuilder, GlobMatcher};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::{Path},
    sync::Arc,
};
use crate::constants::{DEFAULT_EXCLUSIONS, UNSUPPORTED_EXTENSIONS};

pub struct DirectoryProcessor {
    extensions: Arc<Vec<String>>,
    excluded_matchers: Arc<Vec<GlobMatcher>>,
    suppress_prompt: bool,
    output: String,
}

fn get_default_exclusions() -> Vec<String> {
    DEFAULT_EXCLUSIONS.iter().map(|s| s.to_string()).collect()
}

fn create_exclusions(excluded_files: Vec<String>) -> Vec<String> {
    let mut exclusions = get_default_exclusions();
    exclusions.extend(excluded_files);
    exclusions.extend(UNSUPPORTED_EXTENSIONS.iter().map(|s| format!("*.{}", s)));

    exclusions
}


impl DirectoryProcessor {
    pub fn new(extensions: Vec<String>, excluded_files: Vec<String>, suppress_prompt: bool, output: String) -> Self {
        let excluded_files = create_exclusions(excluded_files);
        let excluded_matchers: Vec<GlobMatcher> = excluded_files
            .into_iter()
            .map(|pattern| GlobBuilder::new(&pattern).build().unwrap().compile_matcher())
            .collect();

        Self {
            extensions: Arc::new(extensions),
            excluded_matchers: Arc::new(excluded_matchers),
            suppress_prompt,
            output,
        }
    }

    pub fn should_process_file(&self, path: &Path) -> bool {
        if self.extensions.is_empty() && self.excluded_matchers.is_empty() {
            return true;
        }

        // If there are no specified extensions, process all files that are not excluded.
        if self.extensions.is_empty() {
            return !self.excluded_matchers.iter().any(|matcher| matcher.is_match(path));
        }

        path.extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| self.extensions.iter().any(|e| e == ext))
            && !self.excluded_matchers.iter().any(|matcher| matcher.is_match(path))
    }

    pub fn process_and_write_file(
        &self,
        path: &Path,
        writer: &mut BufWriter<File>,
        pb: &ProgressBar,
    ) -> Result<()> {
        match std::fs::read_to_string(path) {
            Ok(content) => {
                writeln!(writer, "\n--- {} ---", path.display())?;
                writeln!(writer, "{}", content)?;
            }
            Err(err) => {
                eprintln!("Non-UTF-8 file or read error for {}: {}", path.display(), err);
            }
        }

        pb.inc(1);

        Ok(())
    }

    pub fn run(&self, directory_path: &Path) -> Result<usize> {
        let pb = ProgressBar::new(0);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] {bar:40.cyan} {percent}%")?
                .progress_chars("=>-"),
        );

        // Walk the directory, filtering files and directories
        let walker = WalkBuilder::new(directory_path)
            .standard_filters(true)
            .build();

        let mut file_paths = Vec::new();

        for entry in walker {
            let entry = match entry {
                Ok(e) => e,
                Err(err) => {
                    eprintln!("Error reading entry: {}", err);
                    continue;
                }
            };

            let path = entry.path();

            // Skip directories matching exclusion patterns
            if path.is_dir() && self.excluded_matchers.iter().any(|matcher| matcher.is_match(path)) {
                continue; // Do not process this directory
            }

            // Process files matching criteria
            if path.is_file() && self.should_process_file(path) {
                file_paths.push(path.to_owned());
            }
        }

        pb.set_length(file_paths.len() as u64);

        let output_file = File::create(self.output.clone())?;
        let mut writer = BufWriter::new(output_file);

        if !self.suppress_prompt {
            writeln!(writer, "This is a .txt file representing an entire directory's contents.")?;
            writeln!(writer, "Each file is separated by a line with its path.\n")?;
        }

        for path in &file_paths {
            self.process_and_write_file(path, &mut writer, &pb)?;
        }

        pb.finish_with_message("Directory processing complete");

        Ok(file_paths.len())
    }
}
