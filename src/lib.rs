use anyhow::{Context, Result};
use globset::{GlobBuilder, GlobMatcher};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct DirectoryProcessor {
    extensions: Arc<Vec<String>>,
    excluded_matchers: Arc<Vec<GlobMatcher>>,
    suppress_prompt: bool,
    output: String,
}

impl DirectoryProcessor {
    pub fn new(extensions: Vec<String>, excluded_files: Vec<String>, suppress_prompt: bool, output: String) -> Self {
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
        let content = read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        writeln!(writer, "\n--- {} ---", path.display())?;
        writeln!(writer, "{}", content)?;

        pb.inc(1);

        Ok(())
    }

    pub fn run(&self, directory_path: &Path) -> Result<Vec<PathBuf>> {
        let pb = ProgressBar::new(0);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{msg} [{elapsed_precise}] {bar:40.cyan} {percent}%")?
                .progress_chars("=>-"),
        );

        let walker = WalkBuilder::new(directory_path).standard_filters(true).build();

        let file_paths: Vec<PathBuf> = walker
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                let path = entry.path();
                path.is_file() && self.should_process_file(path)
            })
            .map(|entry| entry.path().to_owned())
            .collect();

        pb.set_length(file_paths.len() as u64);

        let output_file = File::create(self.output.clone())?;
        let mut writer = BufWriter::new(output_file);

        if !self.suppress_prompt {
            writeln!(writer, "This is a .txt file representing an entire directory's contents.")?;
            writeln!(writer, "Each file is separated by a line with its path.\n")?;
        }

        for path in file_paths.iter().clone() {
            self.process_and_write_file(&path, &mut writer, &pb)?;
        }

        pb.finish_with_message("Directory processing complete");

        Ok(file_paths)
    }
}
