# vreq-rs

vreq-rs is a lightweight Rust CLI for generating and managing Python
`requirements.txt` files directly from virtual environments.

It is designed to be fast, simple, and dependency-free on the Python
side, while providing a clean workflow for everyday development.

## Features

-   Generate `requirements.txt` from the active virtual environment
-   Default mode includes only direct dependencies
-   Optional full dependency tree generation
-   Sync environment from `requirements.txt`
-   Automatically pin versions when installing from unpinned files
-   Detect project root automatically
-   Works from any subdirectory inside a project

## Installation

### Using Cargo

``` bash
cargo install --path .
```

Make sure `~/.cargo/bin` is in your PATH.

### Manual

``` bash
cargo build --release
cp target/release/vreq /usr/local/bin/
```

## Usage

Activate your Python virtual environment before using vreq.

``` bash
source venv/bin/activate
```

### Generate requirements file

``` bash
vreq req gen
```

### Generate full dependency tree

``` bash
vreq req gen --all
```

### Sync environment

``` bash
vreq req sync
```

### Sync with full dependency regeneration

``` bash
vreq req sync --all
```

## How It Works

-   Uses the active virtual environment (`VIRTUAL_ENV`)
-   Calls Python's `pip` internally
-   Detects project root by searching for:
    -   `requirements.txt`
    -   `pyproject.toml`
    -   `.git` (fallback)
-   Writes output to the detected project root

## Example Workflow

``` bash
pip install fastapi uvicorn

vreq req gen
```

Later:

``` bash
vreq req sync
```

## Design Goals

-   Minimal configuration
-   Fast execution
-   Clear and predictable behavior
-   No lock-in to specific Python tooling

## Limitations

-   Does not track dependency history
-   `pip list --not-required` may not perfectly represent user-installed
    packages
-   Does not yet support advanced dependency resolution

## Future Improvements

-   `vreq req check`
-   `vreq req fix`
-   Multiple requirement files support
-   Better version constraint handling
-   JSON output for CI/CD
-   Improved project detection
-   Optional lock file support

## License

MIT