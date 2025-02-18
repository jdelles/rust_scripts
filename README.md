# Collection of Scripts I'm Using

This repository contains a collection of scripts that I use for various tasks. Below are the details of the current scripts included in this repository.

## Captain's Log Script

This Rust script automates the process of creating a new journal file each day. It generates a markdown file with a template for your daily journal, including sections for today's log, looking ahead, and observations. The script uses the `chrono` crate to get the current date and time, and creates a file path with the current date. It then creates a new file at that path with the journal template.

### Prerequisites

To run the script, you need to have Rust installed on your system. You can install Rust by following the instructions on the official [Rust website](https://www.rust-lang.org/learn/get-started).

### Usage

1. Create a new Rust project:
    ```sh
    cargo new captains-log-script
    ```

2. Navigate to the project directory:
    ```sh
    cd captains-log-script
    ```

3. Add `chrono` as a dependency in `Cargo.toml`:
    ```toml
    [dependencies]
    chrono = "0.4"
    ```

4. Replace the contents of `src/main.rs` with the script provided.

5. Run the script:
    ```sh
    cargo run
    ```

The script will create a new journal file in the `/captains-log/journals/<year>/` directory with the current date as the filename.

### Example

Running the script on February 17, 2025, will create a file named `2025-02-17.md` in the `/captains-log/journals/2025/` directory with the following content:

```markdown
# Captain's Log - February 17, 2025

## Today's Log
- 
- 
- 
- 
- 
- 
- 
- 

## Looking Ahead
- 
- 
- 
- 

## Observations
- 
- 
- 
```

The script checks if a file with the same name already exists and aborts if it does, to prevent overwriting existing journal entries.

## Pre-commit Hook Script

This pre-commit hook script ensures that Rust code is properly formatted before committing. It searches for all `Cargo.toml` files within the repository, changes to their respective directories, runs `cargo fmt` to format the code, stages any changes made by `cargo fmt`, and then returns to the root directory.

### Usage

To use this pre-commit hook, save the script as `pre-commit` in the `.git/hooks` directory of your repository and make it executable:

```sh
chmod +x .git/hooks/pre-commit
```

The script will automatically run cargo fmt on all Rust code in the repository before each commit, ensuring that the code is properly formatted.