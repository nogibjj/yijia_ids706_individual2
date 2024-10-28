# yijia_ids706_individual2

## Rust Template
This Rust project demonstrates Extract, Transform, Load (ETL) operations, connects to a SQLite database, and performs CRUD operations on weather data for the Durham region.

## CI/CD Badge
[![Rust CI](https://github.com/nogibjj/yijia_ids706_individual2/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/yijia_ids706_individual2/actions/workflows/ci.yml)

##  Using LLM 
This project was adapted from a previous Python project (Mini Project 5). I used GitHub Copilot to assist in converting the codebase from Python to Rust. While some fix and some adaptations were necessary due to language differences, like switching from passing a database path to using direct database connections to handle concurrency. ChatGPT was also used to troubleshoot bugs and optimize commands.

## File Structure

- **`.devcontainer/`**: Contains the development container configuration (`devcontainer.json` and a Dockerfile) to ensure a consistent development environment. From (https://github.com/johncoogan53/Rust-Template/blob/main/.devcontainer/Dockerfile)
- **`Makefile`**: Commands for building, formatting, linting, and testing the project.
- **`.github/workflows/`**: Contains CI/CD configurations for formating, linting, building, testing, and deploying the Rust binary.
- **`src/`**: Contains the main program files, including main.rs and lib.rs with core logic.
- **`tests/`**: Contains test_main.rs, testing CRUD operations and ETL functions.
- **`Cargo.toml`** Specifies project dependencies and configuration.

## Installation

### 1. Prerequisites
- Option 1: Run Locally with Rust
    - Install Rust and Cargo.

- Option 2: Download Prebuilt Binary (No Rust Installation Required)
    - The binary is automatically built during the CI/CD process and can be downloaded directly from the GitHub Actions Artifacts section.
    - No need to install Rust on your local machine.
      
### 2. Download Prebuilt Binary (No Rust Required)
Download the prebuilt binary artifact and ensure using a Linux environment.
Here’s how to get the binary:
1. Go to the Actions tab in the repository.
2. Select the latest workflow run.
3. Scroll down to Artifacts and download the yijia_ids706_individual2.
4. Move the binary to a Linux environment, like GitHub Codespaces or a Linux VM, to run the tool."

**Usage of Binary:** 
```sh
chmod +x yijia_ids706_individual2  # Make the binary executable
./yijia_ids706_individual2  # Runs ETL and CRUD operations as defined in main.rs
```

### 3. Running Locally with Rust Installed
#### Steps
1. Clone the repository:

```sh
git clone git@github.com:nogibjj/yijia_ids706_individual2.git
```

2. (Optional): Open the repository in Visual Studio Code and reopen it in a container using the .devcontainer configuration to ensure a consistent development environment.

3. Build the project:
```sh
make build 
```

4. Run the project:
```sh
make run 
```
This command performs the following steps:
1. Sets up file paths.
2. Ensures the data directory exists.
3. Downloads the weather dataset.
4. Connects to the SQLite database.
5. Demonstrates CRUD operations on the dataset.

## Optimized Rust Binary Generation
This project includes a GitHub Actions pipeline that generates an optimized Rust binary. The following section explains how this binary is generated and how to access it.

### GitHub Actions Pipeline: Binary Generation
The ci.yml workflow file performs the following steps:

1. **Build:** Executes cargo build --release to generate an optimized binary.
2. **Upload Binary as an Artifact:** Uploads the optimized binary to GitHub Actions for easy access.
```sh
    - name: Build
      run: make build 
      
    - name: Upload binary as an artifact
      uses: actions/upload-artifact@v4
      with:
        name: rust_stats_tool_binary
        path: target/release/yijia_ids706_individual2
```

## Available Commands
Here is a list of available commands using the Makefile:
```sh
make format  # Formats Rust files using `cargo fmt`.
make lint    # Lints Rust files using `cargo clippy`.
make test    # Runs tests using `cargo test`.
make build   # Builds the project in release mode using `cargo build --release`.
make run     # Runs the tool in the terminal.
```

## CI/CD Pipeline
This project uses GitHub Actions for continuous integration. The pipeline automatically:

1. Checks formatting using cargo fmt.
2. Lints the code with cargo clippy.
3. Runs tests using cargo test.
4. Builds the project in release mode using cargo build --release.

## Demo
https://youtu.be/Udh-4VOadMI
