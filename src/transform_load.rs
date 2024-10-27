use rusqlite::{params, Connection, Result};

pub fn transform_load(dataset: &str) -> Result<String> {
    // Connect to SQLite database (creates WeatherDB.db if it doesn't exist)
    let conn = Connection::open("WeatherDB.db")?;

    // Drop existing WeatherDB table if it exists, then create a new table
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

    // Open CSV file and create a CSV reader with headers
    let mut rdr = csv::Reader::from_path(dataset).expect("Failed to read dataset");

    // Prepare statement for inserting rows into SQLite table
    let mut stmt = conn.prepare(
        "INSERT INTO WeatherDB (Date, Temperature_Minimum, Temperature_Maximum, Precipitation, Snowfall, Snow_Depth, Average_Wind_Speed)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )?;

    for result in rdr.records() {
        match result {
            Ok(record) => {
                // Parse each field, handling parsing errors with default values
                let date = &record[0];
                let temp_min = record[1].parse::<f64>().unwrap_or(0.0);
                let temp_max = record[2].parse::<f64>().unwrap_or(0.0);
                let precipitation = record[3].parse::<f64>().unwrap_or(0.0);
                let snowfall = record[4].parse::<f64>().unwrap_or(0.0);
                let snow_depth = record[5].parse::<f64>().unwrap_or(0.0);
                let avg_wind_speed = record[6].parse::<f64>().unwrap_or(0.0);

                stmt.execute(params![
                    date,
                    temp_min,
                    temp_max,
                    precipitation,
                    snowfall,
                    snow_depth,
                    avg_wind_speed
                ])?;
            }
            Err(err) => {
                eprintln!("Error reading CSV record: {:?}", err);
            }
        }
    }

    println!("Data loaded into WeatherDB.db successfully.");
    Ok("WeatherDB.db".to_string())
}
