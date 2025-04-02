mod constants;

use crate::constants::{DEFAULT_EXCLUSIONS, UNSUPPORTED_EXTENSIONS};
use anyhow::Result;
use globset::{GlobBuilder, GlobMatcher};
use ignore::WalkBuilder;
use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use std::{
    fs::File,
    io::{BufRead, BufWriter, Write},
    path::Path,
    sync::Arc,
};

#[derive(Debug)]
pub enum Filter {
    FileName(String),        // Matches file name
    PathContains(String),    // Matches a substring in the path
    ContentContains(String), // Matches a substring in the file content
}

pub struct DirectoryProcessor {
    extensions: Arc<Vec<String>>,
    excluded_matchers: Arc<Vec<GlobMatcher>>,
    suppress_prompt: bool,
    output: String,
    force: bool,
    filters: Vec<Filter>,
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
    pub fn new(
        extensions: Vec<String>,
        excluded_files: Vec<String>,
        suppress_prompt: bool,
        output: String,
        force: bool,
        filters: Vec<Filter>,
    ) -> Self {
        let excluded_files = create_exclusions(excluded_files);
        let excluded_matchers: Vec<GlobMatcher> = excluded_files
            .into_iter()
            .map(|pattern| {
                GlobBuilder::new(&pattern)
                    .build()
                    .unwrap()
                    .compile_matcher()
            })
            .collect();

        Self {
            extensions: Arc::new(extensions),
            excluded_matchers: Arc::new(excluded_matchers),
            suppress_prompt,
            output,
            force,
            filters,
        }
    }

    pub fn run(&self, directory_path: &Path) -> Result<usize> {
        // Validate the output file if provided
        match self.validate_output_file(
            self.output.clone(),
            self.force,
            &mut std::io::BufReader::new(std::io::stdin().lock()),
            &mut std::io::stdout(),
        ) {
            Ok(false) => std::process::exit(0), // exit if 'n'
            Err(err) => eprintln!("Error during output file validation: {}", err),
            _ => {}
        }

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
            if path.is_dir()
                && self
                    .excluded_matchers
                    .iter()
                    .any(|matcher| matcher.is_match(path))
            {
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
            writeln!(
                writer,
                "This is a .txt file representing an entire directory's contents."
            )?;
            writeln!(writer, "Each file is separated by a line with its path.\n")?;
        }

        for path in &file_paths {
            self.process_and_write_file(path, &mut writer, &pb)?;
        }

        pb.finish_with_message("Directory processing complete");

        Ok(file_paths.len())
    }

    pub fn should_process_file(&self, path: &Path) -> bool {
        // Check if the path is empty
        if path.to_str().unwrap_or("").is_empty() {
            return false;
        }

        // Check file name for exclusion patterns
        let file_name = path.file_name().and_then(|name| name.to_str());
        if let Some(name) = file_name {
            if self
                .excluded_matchers
                .iter()
                .any(|matcher| matcher.is_match(name))
            {
                debug!("Excluding file due to name pattern: {}", name);
                return false;
            }
        }

        let path_str = path.to_string_lossy();

        // If there are no filters, and not excluded, always include
        if self.filters.is_empty() {
            if self.extensions.is_empty() {
                debug!("Processing file without filters: {}", path_str);
                return true;
            }
            debug!("Checking extension for file: {}", path_str);
            return path
                .extension()
                .and_then(|ext| ext.to_str())
                .map_or(false, |ext| self.extensions.iter().any(|e| e == ext));
        }

        // Apply advanced filters. Now OR logic
        let mut matched_filter = false;
        for filter in &self.filters {
            match filter {
                Filter::FileName(pattern) => {
                    if let Some(name) = file_name {
                        if name.contains(pattern) {
                            matched_filter = true;
                            break; // If one filter matches, no need to check others for this type
                        }
                    }
                }
                Filter::PathContains(substring) => {
                    if path_str.contains(substring) {
                        matched_filter = true;
                        break; // If one filter matches, no need to check others for this type
                    }
                }
                Filter::ContentContains(_) => {
                    matched_filter = true; // Content filter is checked later in process_and_write_file
                    break;
                }
            }
        }

        if !matched_filter {
            return false;
        }

        // If no extensions are specified, process all files that pass the filters
        if self.extensions.is_empty() {
            debug!("Processing file with filters: {}", path_str);
            return true;
        }

        // Match files by extension
        path.extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| self.extensions.iter().any(|e| e == ext))
    }

    pub fn validate_output_file<R: BufRead, W: Write>(
        &self,
        output_path: String,
        force: bool,
        reader: &mut R,
        writer: &mut W,
    ) -> std::io::Result<bool> {
        let path = std::path::Path::new(&output_path);

        if path.exists() {
            if force {
                return Ok(true);
            }

            let file = File::open(&output_path)?;
            let mut buf_reader = std::io::BufReader::new(&file);
            let mut first_line = String::new();

            // Read the first line of the file and handle other files
            match buf_reader.read_line(&mut first_line) {
                Ok(bytes_read) => {
                    if bytes_read > 0
                        && first_line.contains(
                            "This is a .txt file representing an entire directory's contents.",
                        )
                    {
                        return Ok(true);
                    }
                }
                Err(err) if err.kind() == std::io::ErrorKind::InvalidData => {
                    eprintln!("Binary output file detected (Invalid UTF-8).");
                }
                Err(err) => {
                    eprintln!("I/O error while reading output file: {}", err);
                    return Err(err.into());
                }
            }

            debug!("Opened output file '{}', reading file size.", output_path);

            // Check metadata for file size
            if let Ok(metadata) = std::fs::metadata(&output_path) {
                if metadata.len() > 0 {
                    write!(
                        writer,
                        "Output file '{}' already exists and contains data. Overwrite? (y/n): ",
                        output_path
                    )?;
                    writer.flush()?;

                    let mut input = String::new();
                    reader.read_line(&mut input)?;

                    if !input.trim().eq_ignore_ascii_case("y") {
                        writeln!(writer, "Operation cancelled.")?;
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

    fn process_and_write_file(
        &self,
        path: &Path,
        writer: &mut BufWriter<File>,
        pb: &ProgressBar,
    ) -> Result<()> {
        let content = match std::fs::read_to_string(path) {
            Ok(content) => content,
            Err(err) => {
                eprintln!(
                    "Non-UTF-8 file or read error for {}: {}",
                    path.display(),
                    err
                );
                return Err(err.into());
            }
        };

        debug!("Writing content for file: {}", path.display());

        // If there are no content filters, write the content
        if self.filters.is_empty()
            || self.filters.iter().any(|f| match f {
                Filter::ContentContains(ref s) => content.contains(s),
                _ => false,
            })
        {
            writeln!(writer, "\n--- {} ---", path.display())?;
            writeln!(writer, "{}", content)?;
            pb.inc(1);
        }

        Ok(())
    }
}
