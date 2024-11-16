# `codepack` (v0.6.2)

[![crates.io](https://img.shields.io/crates/v/codepack.svg)](https://crates.io/crates/codepack)

**`codepack`** transforms directories into *LLM-friendly* text files.

A *lightning-fast* tool that converts the contents of a directory into a single, LLM-friendly text file, making it easy to process and analyze data with Large Language Models.

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

## Usage
```bash
codepack [OPTIONS] <DIRECTORY_PATH>

Convert local directory contents into a single text file, useful for processing by an LLM.

Options:
-h, --help              Show this message
-e, --extension <EXT>   Include files with the specified extension(s) (e.g., -e rs -e toml)
-o, --output <OUTPUT>   Specify the output file path (optional)
-s, --suppress-prompt   Suppress the description of the file format in the output
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

