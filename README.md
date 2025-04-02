# Slugify

A simple command-line tool written in Rust that converts text into URL-friendly slugs.

## What is a slug?

A slug is a URL-friendly version of a string, typically used in web applications for creating clean, readable URLs. Slugs are lowercase, contain only alphanumeric characters, underscores, or hyphens, and have no leading or trailing whitespace or special characters.

## Features

- Converts text to lowercase
- Replaces spaces with hyphens
- Removes non-alphanumeric characters (except underscores and hyphens)
- Collapses multiple hyphens into a single hyphen
- Trims leading and trailing whitespace, hyphens, and underscores

## Installation

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs/))

### Building from source

```bash
# Clone the repository
git clone https://github.com/yourusername/slugify.git
cd slugify

# Build the project
cargo build --release

# The binary will be available at target/release/slugify
```

## Usage

```bash
slugify [STRINGS]...
```

Where `[STRINGS]...` are one or more strings to be slugified. Multiple strings will be joined with spaces before slugification.

### Examples

```bash
# Basic usage
$ slugify Hello World
hello-world

# Multiple arguments
$ slugify The Quick Brown Fox
the-quick-brown-fox

# Special characters
$ slugify "Hello, World! How are you?"
hello-world-how-are-you

# Multiple hyphens get collapsed
$ slugify "Hello -- World"
hello-world
```

## Help

```bash
slugify --help
```
