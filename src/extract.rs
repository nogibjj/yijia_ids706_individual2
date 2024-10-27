use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;

pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Send a GET request to the URL
    let response = get(url)?;
    let mut file = File::create(file_path)?;

    // Write the response content to the specified file path
    file.write_all(&response.bytes()?)?;
    println!("File downloaded to {}", file_path);
    Ok(())
}
