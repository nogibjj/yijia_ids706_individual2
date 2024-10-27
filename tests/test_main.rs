#[cfg(test)]
mod tests {
    use yijia_ids706_individual2::{extract, query, transform_load};
    use rusqlite::{Connection, Result, OptionalExtension};
    use std::fs;

    // Ensure the database is loaded
    fn ensure_database_loaded() -> Result<()> {
        let conn = Connection::open("WeatherDB.db")?;
        let table_exists: bool = conn.query_row(
            "SELECT EXISTS (SELECT name FROM sqlite_master WHERE type='table' AND name='WeatherDB')",
            [],
            |row| row.get(0),
        )?;
        
        if !table_exists {
            let dataset_path = "rdu-weather-history.csv";
            transform_load::transform_load(dataset_path)?;
        }
        Ok(())
    }

    #[test]
    fn test_extract() {
        let url = "https://raw.githubusercontent.com/nogibjj/yijia_ids706_miniProj3/refs/heads/main/rdu-weather-history.csv";
        let file_path = "rdu-weather-history.csv";
        
        let result = extract::extract(url, file_path);
        assert!(result.is_ok(), "Failed to download file: {:?}", result.unwrap_err());
        assert!(fs::metadata(file_path).is_ok(), "File not found after extraction.");
        println!("Extract test passed!");
    }

    #[test]
    fn test_transform_load() -> Result<()> {
        let dataset_path = "rdu-weather-history.csv";
        let db_result = transform_load::transform_load(dataset_path);
        
        assert!(db_result.is_ok(), "Failed to load data into SQLite: {:?}", db_result.unwrap_err());
        let conn = Connection::open("WeatherDB.db")?;
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM WeatherDB", [], |row| row.get(0))?;
        assert!(count > 0, "No data found in the database.");
        println!("Load test passed!");
        Ok(())
    }

    #[test]
    fn test_create_entry() -> Result<()> {
        ensure_database_loaded()?;
        let mut conn = Connection::open("WeatherDB.db")?;
        let tx = conn.transaction()?;

        let new_record = ["2024-10-04", "60.0", "85.0", "0.2", "0.0", "0.0", "12.5"];
        query::create_entry("WeatherDB.db", &new_record)?;

        let result: (String, f64, f64, f64, f64, f64, f64) = tx.query_row(
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
        tx.rollback()?;  // Rollback transaction to discard changes
        println!("Create test passed!");
        Ok(())
    }

    #[test]
    fn test_read_entry() -> Result<()> {
        ensure_database_loaded()?;
        let mut conn = Connection::open("WeatherDB.db")?;
        let tx = conn.transaction()?;

        let result = query::read_entry("WeatherDB.db", "2022-01-08")?;
        assert!(!result.is_empty(), "Read test failed: No data found.");

        tx.rollback()?; // Rollback transaction to discard any inadvertent changes
        println!("Read test passed!");
        Ok(())
    }

    #[test]
    fn test_update_entry() -> Result<()> {
        ensure_database_loaded()?;
        let mut conn = Connection::open("WeatherDB.db")?;
        let tx = conn.transaction()?;

        let updated_data = ["65.0", "90.0", "0.1", "0.0", "0.0", "10.0"];
        query::update_entry("WeatherDB.db", "2022-01-17", &updated_data)?;

        let result: (String, f64, f64, f64, f64, f64, f64) = tx.query_row(
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
        assert_eq!(result.1, updated_data[0].parse::<f64>().unwrap(), "Update test failed: Data mismatch.");

        tx.rollback()?; // Rollback transaction to discard changes
        println!("Update test passed!");
        Ok(())
    }

    #[test]
    fn test_delete_entry() -> Result<()> {
        ensure_database_loaded()?;
        let mut conn = Connection::open("WeatherDB.db")?;
        let tx = conn.transaction()?;

        query::delete_entry("WeatherDB.db", "2022-01-26")?;

        let result = tx.query_row(
            "SELECT * FROM WeatherDB WHERE Date = ?",
            ["2022-01-26"],
            |row| Ok(row.get::<_, String>(0)),
        ).optional()?;
        
        assert!(result.is_none(), "Delete test failed: Record still exists.");
        
        tx.rollback()?; // Rollback transaction to discard changes
        println!("Delete test passed!");
        Ok(())
    }
}
