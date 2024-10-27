use rusqlite::{params, Connection, Result};

pub fn create_entry(database: &str, data: &[&str]) -> Result<()> {
    let conn = Connection::open(database)?;
    let query = "
        INSERT INTO WeatherDB (Date, Temperature_Minimum, Temperature_Maximum, Precipitation, Snowfall, Snow_Depth, Average_Wind_Speed)
        VALUES (?, ?, ?, ?, ?, ?, ?)";
    conn.execute(
        query,
        params![data[0], data[1], data[2], data[3], data[4], data[5], data[6]],
    )?;
    Ok(())
}

pub fn read_entry(
    database: &str,
    specific_date: &str,
) -> Result<Vec<(String, f64, f64, f64, f64, f64, f64)>> {
    let conn = Connection::open(database)?;
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

pub fn update_entry(database: &str, date: &str, new_data: &[&str]) -> Result<()> {
    let conn = Connection::open(database)?;
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

pub fn delete_entry(database: &str, date: &str) -> Result<()> {
    let conn = Connection::open(database)?;
    let query = "DELETE FROM WeatherDB WHERE Date=?";
    conn.execute(query, params![date])?;
    Ok(())
}
