use rusqlite::{Connection, Result};

pub fn create_connection() -> Connection{
    let conn = Connection::open("my_database.db").unwrap();
    conn
}

pub fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_settings (
            id INTEGER PRIMARY KEY,
            email TEXT NOT NULL,
            minimum_length INTEGER DEFAULT 8,
            include_numbers BOOLEAN DEFAULT false,
            include_special BOOLEAN DEFAULT false,
            include_ucase BOOLEAN DEFAULT false,
            use_words BOOLEAN DEFAULT true
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
        )",
        []
        )?;
    Ok(())
}
fn modify_single_setting(conn: &Connection, email: String, col: String, val: String) -> Result<()> {
    //TODO: 'val' param shouldn't be string; should be enum as outlined in cli module
    conn.execute(
        "UPDATE password_settings (
            SET ?2 = ?3
        )
        WHERE email = ?1;
        ",
        &[&email, &col, &val]
        )?;
    Ok(())
}

