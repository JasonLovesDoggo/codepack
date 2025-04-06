# `codepack`

[![crates.io](https://img.shields.io/crates/v/codepack.svg)](https://crates.io/crates/codepack) [![Web](https://github.com/JasonLovesDoggo/codepack/actions/workflows/web.yml/badge.svg)](https://github.com/JasonLovesDoggo/codepack/actions/workflows/web.yml) [![Release](https://github.com/JasonLovesDoggo/codepack/actions/workflows/release.yml/badge.svg)](https://github.com/JasonLovesDoggo/codepack/actions/workflows/release.yml)

**`codepack`** transforms directories into _LLM-friendly_ text files.

A _lightning-fast_ tool that converts the contents of a directory into a single, LLM-friendly text file, making it easy to process and analyze data with Large Language Models.

## Installation

For detailed installation instructions, please visit the [Codepack documentation](https://codepack.jasoncameron.dev/).

Alternatively, you can follow the instructions below to install `codepack`.

### Pre-built Binaries (Recommended)

Please see [the documentation](https://codepack.jasoncameron.dev)

### Installation via Cargo (Optional)

Alternatively, you can install `codepack` from source with Rust's package manager `cargo`:

```sh
cargo install codepack
```

This will download, build, and install the latest version of codepack.

## Quickstart

Once installed, simply run:

```bash
codepack /path/to/directory
```

By default, codepack will process the directory and output a .txt file with the contents of the files inside the directory. If you don't specify an output file, it will generate one based on the directory name and the number of files processed.

## Features

- **Lightning Fast**: `codepack` is optimized for speed, ensuring that even large directories are processed efficiently.
- **Customizable Output**: Specify the output file name with the `-o` option, or let `codepack` generate one for you.
- **Selective File Processing**: Use the `-e` or `--extension` flag to include specific file types (e.g., `.rs`, `.toml`).
- **Suppress Output Prompt**: If you don’t want the default prompt in your output file, use the `--suppress-prompt` option.
- **Powerful Filtering**: Filter files based on file names, paths, and content using the `-f` or `--filter` option.

### Filtering with codepack

Codepack supports three types of filters:

1. File Name Filters: Filter files based on their names using the file.name= prefix followed by the pattern to match.

> Example: `codepack -f "file.name=main.py" /path/to/project` (includes only files named "main.py")

2. Path Contains Filters: Filter files based on a substring present anywhere in their path using the path.contains= prefix followed by the substring.

> Example: `codepack -f "path.contains=src" .` (includes only files within the "src" directory and its subdirectories)

3. Content Contains Filters: Filter files based on a substring present in their content using the content.contains= prefix followed by the substring.

Note: Content filters are applied after the file has been read, so this can be slower than other filter types.

> Example: `codepack -f "content.contains=function" /path/to/code` (includes only files containing the word "function")

You can combine multiple filters using multiple `-f` or `--filter` options. Codepack uses `OR` logic for filtering, so a file will be included if it matches any of the provided filters.

## Usage

```bash
codepack [OPTIONS] <DIRECTORY_PATH>

Convert local directory contents into a single text file, useful for processing by an LLM.

Options:
-h, --help              Show this message
-e, --extension <EXT>   Include files with the specified extension(s) (e.g., -e rs -e toml)
-o, --output <OUTPUT>   Specify the output file path (optional)
--suppress-prompt   Suppress the description of the file format in the output
--force   Overwrite output file without confirmation
-f,  --filter <FILTER>      Filter files based on name, path, or content (e.g., -f "file.name=main.rs").
```

Examples
Convert a directory to a .txt file with a custom output name:

```bash
codepack /path/to/my/code -o my_code.txt
```

Process only `.rs` and `.toml` files from a directory:

```bash
codepack /path/to/my/code -e rs -e toml
```

## Contributing

We welcome contributions to codepack! Please feel free to submit issues or pull requests on GitHub.

## License

codepack is distributed under the terms of the GPL-3.0 License. See the [LICENSE](./LICENSE) file for details.
