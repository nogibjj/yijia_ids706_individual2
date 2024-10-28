use rusqlite::{params, Connection, Result};
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;
use csv::Reader;

// Define a type alias 
type WeatherData = Vec<(String, f64, f64, f64, f64, f64, f64)>;

// Function to download a file from a URL and save it locally
pub fn extract(url: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut file = File::create(file_path)?;
    file.write_all(&response.bytes()?)?;
    println!("File downloaded to {}", file_path);
    Ok(())
}

// Function to load data from a CSV file into the SQLite database
pub fn transform_load(conn: &Connection, dataset: &str) -> Result<String> {
    // Drop the table if it exists and create a new one
    conn.execute("DROP TABLE IF EXISTS WeatherDB", [])?;
    conn.execute(
        "CREATE TABLE WeatherDB (
            Date TEXT, 
            Temperature_Minimum REAL, 
            Temperature_Maximum REAL, 
            Precipitation REAL, 
            Snowfall REAL, 
            Snow_Depth REAL, 
            Average_Wind_Speed REAL
        )",
        [],
    )?;

    // Open the CSV file and read records
    let mut rdr = Reader::from_path(dataset).expect("Failed to read dataset");

    // Prepare the statement for inserting rows
    let mut stmt = conn.prepare(
        "INSERT INTO WeatherDB (Date, Temperature_Minimum, Temperature_Maximum, Precipitation, Snowfall, Snow_Depth, Average_Wind_Speed)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                stmt.execute(params![
                    &record[0], 
                    &record[1].parse::<f64>().unwrap_or(0.0), 
                    &record[2].parse::<f64>().unwrap_or(0.0), 
                    &record[3].parse::<f64>().unwrap_or(0.0), 
                    &record[4].parse::<f64>().unwrap_or(0.0), 
                    &record[5].parse::<f64>().unwrap_or(0.0), 
                    &record[6].parse::<f64>().unwrap_or(0.0)  
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    println!("Data loaded into WeatherDB successfully.");
    Ok("WeatherDB loaded".to_string())
}

// Function to insert a record into the database
pub fn create_entry(conn: &Connection, data: &[&str]) -> Result<()> {
    let query = "
        INSERT INTO WeatherDB (Date, Temperature_Minimum, Temperature_Maximum, Precipitation, Snowfall, Snow_Depth, Average_Wind_Speed)
        VALUES (?, ?, ?, ?, ?, ?, ?)";
    conn.execute(query, params![data[0], data[1], data[2], data[3], data[4], data[5], data[6]])?;
    Ok(())
}

// Function to retrieve records by date
pub fn read_entry(conn: &Connection, specific_date: &str) -> Result<WeatherData> {
    let mut stmt = conn.prepare("SELECT * FROM WeatherDB WHERE Date = ?")?;
    let weather_data = stmt
        .query_map(params![specific_date], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(weather_data)
}

// Function to update a record by date
pub fn update_entry(conn: &Connection, date: &str, new_data: &[&str]) -> Result<()> {
    let query = "
        UPDATE WeatherDB 
        SET Temperature_Minimum=?, Temperature_Maximum=?, Precipitation=?, Snowfall=?, Snow_Depth=?, Average_Wind_Speed=? 
        WHERE Date=?";
    conn.execute(
        query,
        params![
            new_data[0],
            new_data[1],
            new_data[2],
            new_data[3],
            new_data[4],
            new_data[5],
            date
        ],
    )?;
    Ok(())
}

// Function to delete a record by date
pub fn delete_entry(conn: &Connection, date: &str) -> Result<()> {
    let query = "DELETE FROM WeatherDB WHERE Date=?";
    conn.execute(query, params![date])?;
    Ok(())
}
