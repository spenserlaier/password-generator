use rusqlite::{Connection, Result, params};
use crate::generation_logic;

pub fn create_connection() -> Connection{
    let conn = Connection::open("profiles_database.db").unwrap();
    conn
}

pub fn initialize_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS password_settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            profile_name TEXT UNIQUE NOT NULL,
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
fn modify_single_setting(conn: &Connection, profile_name: &String, col: &String, val: &String) -> Result<()> {
    //TODO: 'val' param shouldn't be string; should be enum as outlined in cli module
    conn.execute(
        "UPDATE password_settings (
            SET ?2 = ?3
        )
        WHERE profile_name = ?1;
        ",
        &[profile_name, col, val]
        )?;
    Ok(())
}
fn delete_user_profile(conn: &Connection, profile_name: &String) -> Result<()> {
    conn.execute(
        "DELETE FROM password_settings 
        WHERE profile_name = ?1;
        ",
        &[profile_name]
        )?;
    Ok(())
}
pub fn print_single_profile(conn: &Connection, profile_name: &String) {
    let profile_settings = retrieve_profile_settings(&conn, profile_name);
    match profile_settings {
        Some(profile_settings) => {
            println!("Printing settings for profile: {}", profile_name);
            println!("Minimum length: {}", profile_settings.minimum_length);
            println!("Includes numbers? : {}", profile_settings.include_numbers);
            println!("Includes special characters? : {}", profile_settings.include_special);
            println!("Includes uppercase characters? : {}", profile_settings.include_ucase);
            println!("Uses dictionary words instead of random lowercase alphabetic chars? : {}",
                     profile_settings.use_words);
        }
        None => {
            println!("Could not find the specified profile for printing.")
        }
    }
}
pub fn insert_user_profile(conn: &Connection, generation_features: &generation_logic::GenerationData) -> Result<()> {
    let result = conn.execute(
        "INSERT INTO password_settings 
        (
        profile_name,
        minimum_length,
        include_numbers,
        include_special,
        include_ucase,
        use_words
        )
        VALUES (
        ?1,
        ?2,
        ?3,
        ?4,
        ?5,
        ?6
        )
        ",
        params![generation_features.profile.as_ref().unwrap(),
        &(generation_features.minimum_length),
        &(generation_features.include_numbers),
        &(generation_features.include_special),
        &(generation_features.include_ucase),
        &(generation_features.use_words)]
        );
    match result {
        Ok(x) => {}
        Err(x) => {
            println!("error when inserting into database: {}", x);
        }
    }
    Ok(())
}
pub fn print_profiles(conn: &Connection) {
    let mut statement = conn.prepare("SELECT profile_name FROM password_settings;").unwrap();
    let profile_iter = statement.query_map([], |row| {
        Ok(row.get::<_, String>(0)?)
    }).unwrap();
    for profile in profile_iter {
        println!("{}", profile.unwrap());
    }
}



pub fn retrieve_profile_settings(conn: &Connection, profile_name: &String) -> Option<generation_logic::GenerationData>{
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
        Ok(generation_data) => {Some(generation_data)}
        Err(x) => {
            println!("error retrieving user profile: ");
            println!("{}", x);
            None
        }
    }
}
#[cfg(test)]
mod tests {
    use super::{
        retrieve_profile_settings,
        create_connection,
        insert_user_profile,
        delete_user_profile,
        initialize_db,
        print_profiles,
    };
    use crate::generation_logic::{
        GenerationData
    };

    #[test]
    fn retrieve_settings_nonexistent_profile_should_return_none() {
        let conn = create_connection();
        initialize_db(&conn).unwrap();
        let res = retrieve_profile_settings(&conn, &String::from("nonexistent_profile_name"));
        match res{
            None => {},
            Some(_) => {panic!();}
        }
    }
    #[test]
    fn insert_new_profile_and_retrieve_settings() {
        let conn = create_connection();
        initialize_db(&conn).unwrap();
        let default_user = GenerationData::new(None, 
                                               None, 
                                               None, 
                                               None, 
                                               None, 
                                               Some(String::from("new_profile_name")), 
                                               None);
        let insertion_result = insert_user_profile(&conn, &default_user);
        let retrieved_profile = retrieve_profile_settings(&conn, &default_user.profile.as_ref().unwrap()).unwrap();
        print_profiles(&conn);
        delete_user_profile(&conn, &default_user.profile.as_ref().unwrap()).unwrap(); // clean up afterwards
        assert_eq!(retrieved_profile, default_user);
    }
}






