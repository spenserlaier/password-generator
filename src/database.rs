use rusqlite::{Connection, Result};
use crate::generation_logic;

pub fn create_connection() -> Connection{
    let conn = Connection::open("profiles_database.db").unwrap();
    conn
}

pub fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_settings (
            id INTEGER PRIMARY KEY,
            profile_name TEXT NOT NULL,
            minimum_length INTEGER DEFAULT 8,
            include_numbers BOOLEAN DEFAULT false,
            include_special BOOLEAN DEFAULT false,
            include_ucase BOOLEAN DEFAULT false,
            use_words BOOLEAN DEFAULT true,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        []
        )?;
    Ok(())
}
fn modify_single_setting(conn: &Connection, profile_name: String, col: String, val: String) -> Result<()> {
    //TODO: 'val' param shouldn't be string; should be enum as outlined in cli module
    conn.execute(
        "UPDATE password_settings (
            SET ?2 = ?3
        )
        WHERE profile_name = ?1;
        ",
        &[&profile_name, &col, &val]
        )?;
    Ok(())
}
fn retrieve_profile_settings(conn: &Connection, profile_name: String) -> Option<generation_logic::GenerationData>{
    let result = conn.query_row(
        "
        SELECT * FROM password_settings
        WHERE profile_name = ?1;
        ",
        &[&profile_name],
        |row| {
            let profile: String = row.get("profile_name").unwrap();
            let minimum_length: usize = row.get("minimum_length").unwrap();
            let include_numbers: bool = row.get("include_numbers").unwrap();
            let include_special: bool = row.get("include_special").unwrap();
            let include_ucase: bool = row.get("include_ucase").unwrap();
            let use_words: bool = row.get("use_words").unwrap();
            Ok(
                generation_logic::GenerationData::new(
                    Some(minimum_length),
                    Some(include_numbers),
                    Some(include_special),
                    Some(include_ucase),
                    Some(use_words),
                    Some(profile),
                    None
                    )
                )
        }
    );
    match result {
        Ok(x) => Some(x),
        _ => None
    }

}




