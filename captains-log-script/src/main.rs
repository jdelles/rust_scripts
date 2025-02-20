/*
    # Captain's Log Script

    This Rust script automates the process of creating a new journal file each day. It generates a markdown file with a template for your daily journal, including sections for today's log, looking ahead, and observations. The script uses the `chrono` crate to get the current date and time, and creates a file path with the current date. It then creates a new file at that path with the journal template.

    ## Prerequisites

    To run the script, you need to have Rust installed on your system. You can install Rust by following the instructions on the official Rust website: https://www.rust-lang.org/learn/get-started

    ## Usage

    Once you have Rust installed, you can create a new Rust project by running the following command in your terminal:

    ```sh
    cargo new captains-log-script
    ```

    Navigate to the project directory:

    ```sh
    cd captains-log-script
    ```

    Replace the contents of [main.rs](http://_vscodecontentref_/0) with the script provided.

    To run the script, use the following command:

    ```sh
    cargo run
    ```

    The script will create a new journal file in the `/captains-log/journals/<year>/` directory with the current date as the filename.

    ## Example

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
*/

use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::prelude::*;

fn main() {
    let today = Local::now().format("%Y-%m-%d").to_string();
    let year = Local::now().format("%Y").to_string();
    let file_path = format!("/Users/jamesdelles/captains-log/journals/{}/{}.md", year, today);

    let path = Path::new(&file_path);
    let display = path.display();

    if path.exists() {
        println!("File {} already exists. Aborting.", display);
        return;
    }

    let template = format!(
        r#"# Captain's Log - {}
    
## Today's Log

- 
- 
- 

## Looking Ahead

- 
- 
- 

## Observations

- 
- 
- 

"#,
        Local::now().format("%B %d, %Y")
    );

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(template.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully created {}", display),
    }
}
