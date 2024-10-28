#[cfg(test)]
mod tests {
    use rusqlite::{Connection, OptionalExtension, Result};
    use yijia_ids706_individual2::{extract, transform_load, create_entry, read_entry, update_entry, delete_entry};
    fn setup_in_memory_db() -> Connection {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory database");
        // Set up the WeatherDB table schema for each test
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
        ).expect("Failed to create WeatherDB table in in-memory database");
        conn
    }

    #[test]
    fn test_extract() {
        let url = "https://raw.githubusercontent.com/nogibjj/yijia_ids706_miniProj3/refs/heads/main/rdu-weather-history.csv";
        let file_path = "rdu-weather-history.csv";

        let result = extract(url, file_path);
        assert!(
            result.is_ok(),
            "Failed to download file: {:?}",
            result.unwrap_err()
        );
        assert!(
            std::fs::metadata(file_path).is_ok(),
            "File not found after extraction."
        );
        println!("Extract test passed!");
    }

    #[test]
    fn test_transform_load() -> Result<()> {
        let conn = setup_in_memory_db();
        let dataset_path = "rdu-weather-history.csv";
        
        let db_result = transform_load(&conn, dataset_path);
        assert!(
            db_result.is_ok(),
            "Failed to load data into SQLite: {:?}",
            db_result.unwrap_err()
        );

        let count: i64 = conn.query_row("SELECT COUNT(*) FROM WeatherDB", [], |row| row.get(0))?;
        assert!(count > 0, "No data found in the database.");
        println!("Load test passed!");
        Ok(())
    }

    #[test]
    fn test_create_entry() -> Result<()> {
        let conn = setup_in_memory_db();
        let new_record = ["2024-10-04", "60.0", "85.0", "0.2", "0.0", "0.0", "12.5"];
        
        create_entry(&conn, &new_record)?;

        let result: (String, f64, f64, f64, f64, f64, f64) = conn.query_row(
            "SELECT * FROM WeatherDB WHERE Date = ?",
            [&new_record[0]],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )?;
        assert_eq!(result.0, new_record[0], "Create test failed: Record mismatch.");
        println!("Create test passed!");
        Ok(())
    }

    #[test]
    fn test_read_entry() -> Result<()> {
        let conn = setup_in_memory_db();
        let new_record = ["2022-01-08", "21.0", "42.0", "0.0", "0.0", "0.0", "3.8"];
        create_entry(&conn, &new_record)?;

        let result = read_entry(&conn, "2022-01-08")?;
        assert!(!result.is_empty(), "Read test failed: No data found.");
        println!("Read test passed!");
        Ok(())
    }

    #[test]
    fn test_update_entry() -> Result<()> {
        let conn = setup_in_memory_db();
        let initial_record = ["2022-01-17", "20.0", "40.0", "0.0", "0.0", "0.0", "5.0"];
        create_entry(&conn, &initial_record)?;

        let updated_data = ["65.0", "90.0", "0.1", "0.0", "0.0", "10.0"];
        update_entry(&conn, "2022-01-17", &updated_data)?;

        let result: (String, f64, f64, f64, f64, f64, f64) = conn.query_row(
            "SELECT * FROM WeatherDB WHERE Date = ?",
            ["2022-01-17"],
            |row| {
                Ok((
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                    row.get(3)?,
                    row.get(4)?,
                    row.get(5)?,
                    row.get(6)?,
                ))
            },
        )?;
        assert_eq!(result.1, 65.0, "Update test failed: Data mismatch.");
        println!("Update test passed!");
        Ok(())
    }

    #[test]
    fn test_delete_entry() -> Result<()> {
        let conn = setup_in_memory_db();
        let new_record = ["2022-01-26", "15.0", "30.0", "0.0", "0.0", "0.0", "4.0"];
        create_entry(&conn, &new_record)?;

        delete_entry(&conn, "2022-01-26")?;

        let result: Option<String> = conn
            .query_row(
                "SELECT Date FROM WeatherDB WHERE Date = ?",
                ["2022-01-26"],
                |row| row.get(0),
            )
            .optional()?;

        assert!(result.is_none(), "Delete test failed: Record still exists.");
        println!("Delete test passed!");
        Ok(())
    }
}
