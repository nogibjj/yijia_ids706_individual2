mod extract;
mod query;
mod transform_load;

use std::fs;
use std::path::Path;

fn main() {
    // Step 1: Set up paths
    let dataset_url = "https://raw.githubusercontent.com/nogibjj/yijia_ids706_miniProj3/refs/heads/main/rdu-weather-history.csv";
    let dataset_path = "data/rdu-weather-history.csv";
    let db_path = "WeatherDB.db";

    // Step 2: Ensure data directory exists
    let data_dir = Path::new("data");
    if !data_dir.exists() {
        match fs::create_dir(data_dir) {
            Ok(_) => println!("Created data directory."),
            Err(e) => eprintln!("Failed to create data directory: {}", e),
        }
    }

    // Step 3: Download the dataset
    match extract::extract(dataset_url, dataset_path) {
        Ok(_) => println!("Dataset downloaded successfully."),
        Err(e) => eprintln!("Error downloading dataset: {}", e),
    }

    // Step 4: Load data into the SQLite database
    match transform_load::transform_load(dataset_path) {
        Ok(_) => println!("Data loaded into SQLite database."),
        Err(e) => eprintln!("Error loading data into SQLite database: {}", e),
    }

    // Step 5: Perform CRUD operations
    // Create an entry
    let new_data = ["2023-01-01", "1.0", "5.0", "0.0", "0.0", "0.0", "3.0"];
    if let Err(e) = query::create_entry(db_path, &new_data) {
        eprintln!("Error creating entry: {}", e);
    } else {
        println!("Entry created successfully.");
    }

    // Read the entry
    match query::read_entry(db_path, "2023-01-01") {
        Ok(entries) => println!("Read entries: {:?}", entries),
        Err(e) => eprintln!("Error reading entries: {}", e),
    }

    // Update the entry
    let updated_data = ["2.0", "6.0", "0.1", "0.0", "0.1", "4.0"];
    if let Err(e) = query::update_entry(db_path, "2023-01-01", &updated_data) {
        eprintln!("Error updating entry: {}", e);
    } else {
        println!("Entry updated successfully.");
    }

    // Delete the entry
    if let Err(e) = query::delete_entry(db_path, "2023-01-01") {
        eprintln!("Error deleting entry: {}", e);
    } else {
        println!("Entry deleted successfully.");
    }
}
