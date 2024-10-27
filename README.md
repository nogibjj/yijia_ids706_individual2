# yijia_ids706_miniProj7

# Rust Template

This project is a simple statistical tool built with Rust that calculates the mean and median of a list of numbers. The tool is packaged with a Makefile for easier management of tasks, including building, testing, linting, and formatting. CI/CD is implemented via GitHub Actions.

## CI/CD Badge
[![Rust CI](https://github.com/nogibjj/yijia_ids706_miniProj7/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/yijia_ids706_miniProj7/actions/workflows/ci.yml)

## File Structure

- **`.devcontainer/`**: Contains the development container configuration (`devcontainer.json` and a Dockerfile) to ensure a consistent development environment. From (https://github.com/johncoogan53/Rust-Template/blob/main/.devcontainer/Dockerfile)
- **`Makefile`**: Provides commands for building, formatting, linting and testing the project.
- **`.github/workflows/`**: Contains the CI/CD pipeline configuration (ci.yml) that triggers on pushes and pull requests, ensuring continuous testing and validation.
- **`src/`**: Contains the source code for the Rust tool, including the main.rs and lib.rs files.
- **`tests/`**: Contains the test cases for the statistical functions, ensuring that the tool works correctly.


## Installation

### 1. Prerequisites
- Option 1: Run Locally with Rust
    - Install Rust and Cargo.

- Option 2: Download Prebuilt Binary (No Rust Installation Required)
    - The binary is automatically built during the CI/CD process and can be downloaded directly from the GitHub Actions Artifacts section.
    - No need to install Rust on your local machine.
      
### 2. Download Prebuilt Binary (No Rust Required)
Download the prebuilt binary artifact and ensure using a Linux environment (e.g., GitHub Codespaces, a Linux VM, or a Linux machine) 

Here’s how to get the binary:
1. Go to the Actions tab in the repository.
2. Select the latest workflow run.
3. Scroll down to Artifacts and download the yijia_ids706_mini_proj7.
4. Move the binary to a Linux environment where you want to run the tool (e.g., GitHub Codespaces or a Linux machine).

**Usage of Binary:** 
```sh
./yijia_ids706_mini_proj7
```
It will prompt you to enter a list of numbers (separated by spaces) and will return the mean and median.

### 3. Running Locally with Rust Installed
### Steps
1. Clone the repository:

```sh
git clone git@github.com:nogibjj/yijia_ids706_miniProj7.git
```

2. (Optional): Open the repository in Visual Studio Code and reopen it in a container using the .devcontainer configuration to ensure a consistent development environment.

3. Build the project:
```sh
make build 
```

4. Run the tool:
```sh
make run 
```
The tool will prompt you to enter a list of numbers (separated by spaces) and will return the mean and median of those numbers.

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
