use crate::auth::decrypt_password;
use crate::auth::encrypt_password;
use rusqlite::{params, Connection, Result};
use std::fs::{self, set_permissions, Permissions};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use dirs::home_dir;

// Helper function to get the database path
fn get_database_path() -> PathBuf {
    let mut db_path = home_dir().expect("Unable to find home directory");
    db_path.push("Library/Application Support/PasswordManager");
    db_path.push("database.db");
    db_path
}

pub(crate) fn create_database() -> Result<(), String> {
    let db_path = get_database_path();

    // Ensure the directory exists, converting any IO error to a string
    if let Some(parent_dir) = db_path.parent() {
        if !parent_dir.exists() {
            fs::create_dir_all(parent_dir).map_err(|e| e.to_string())?;
            #[cfg(unix)]
            set_permissions(parent_dir, Permissions::from_mode(0o700))
                .map_err(|e| e.to_string())?;
        }
    }

    // Check if the file already exists before creating a new database
    if !db_path.exists() {
        let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

        // Set file permissions for the database file (owner-only read/write)
        #[cfg(unix)]
        set_permissions(&db_path, Permissions::from_mode(0o600))
            .map_err(|e| e.to_string())?;

        // Create tables if they do not exist
        conn.execute(
            "CREATE TABLE IF NOT EXISTS master_password (
            master_hash TEXT PRIMARY KEY
            )",
            params![],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            email_username TEXT NOT NULL UNIQUE
            )",
            params![],
        ).map_err(|e| e.to_string())?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id TEXT NOT NULL,
            website TEXT NOT NULL,
            password TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
            )",
            params![],
        ).map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn generate_custom_id(email_username: &str) -> Result<String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path)?;

    // Get the first two characters of the email/username
    let prefix = &email_username[0..2].to_uppercase();

    // Query the database for all IDs that start with the same prefix
    let mut stmt = conn.prepare("SELECT id FROM users WHERE id LIKE ?1")?;
    let like_pattern = format!("{}%", prefix);
    let ids = stmt.query_map([like_pattern], |row| row.get::<_, String>(0))?;

    // Find the highest number associated with this prefix
    let mut max_number = 0;
    for id in ids {
        let id_str = id?;
        let number_part: i32 = id_str[2..].parse().unwrap_or(0);
        if number_part > max_number {
            max_number = number_part;
        }
    }

    // Increment the number part for the new ID
    let new_id = format!("{}{:03}", prefix, max_number + 1);
    Ok(new_id)
}

#[tauri::command]
pub(crate) fn delete_account(account_id: String) -> Result<(), String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM users WHERE id = ?1", params![account_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub(crate) fn delete_password(password_id: i64) -> Result<(), String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM passwords WHERE id = ?1", params![password_id])
        .map_err(|e| e.to_string())?;

    Ok(())
}

// Getters
#[tauri::command]
pub(crate) fn get_password_hash() -> Result<String, String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT master_hash FROM master_password")
        .map_err(|e| e.to_string())?;

    let result = stmt
        .query_row(params![], |row| {
            let master_hash: String = row.get(0)?;
            Ok(master_hash)
        })
        .map_err(|e| e.to_string())?;

    Ok(result) // Return the result
}

#[tauri::command]
pub(crate) fn get_user_data() -> Result<Vec<(String, String)>, String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    // Query to get both id and email_username
    let mut stmt = conn
        .prepare("SELECT id, email_username FROM users")
        .map_err(|e| e.to_string())?;

    // Map over the rows to get both id and email_username
    let user_iter = stmt
        .query_map([], |row| {
            let id: String = row.get(0)?; // Get the id as String
            let email_username: String = row.get(1)?; // Get the email_username as String
            Ok((id, email_username)) // Return a tuple of (id, email_username)
        })
        .map_err(|e| e.to_string())?;

    let mut users = Vec::new();
    for user in user_iter {
        users.push(user.map_err(|e| e.to_string())?);
    }

    Ok(users) // Return a vector of (id, email_username) tuples
}

#[tauri::command]
pub(crate) fn get_account_passwords(account_id: String) -> Result<Vec<(i64, String, String)>, String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, website, password FROM passwords WHERE user_id = ?1")
        .map_err(|e| e.to_string())?;

    let accounts = stmt
        .query_map(params![account_id], |row| {
            let pass_id: i64 = row.get(0)?;
            let website: String = row.get(1)?;
            let password: String = row.get(2)?;
            Ok((pass_id, website, password))
        })
        .map_err(|e| e.to_string())?;

    let master_password = get_password_hash()?;
    let mut result: Vec<(i64, String, String)> = Vec::new();

    for account in accounts {
        match account {
            Ok((pass_id, website, password)) => {
                let parts: Vec<String> = password.split('|').map(|s| s.to_string()).collect();

                if parts.len() >= 3 {
                    let salt = parts[0].clone();
                    let nonce = parts[1].clone();
                    let ciphertext = parts[2].clone();

                    // Assuming `decrypt_password` returns Result<String, Error>
                    match decrypt_password(master_password.clone(), salt, nonce, ciphertext) {
                        Ok(plaintext) => {
                            result.push((pass_id, website, plaintext));
                        }
                        Err(e) => {
                            eprintln!("Error decrypting password for {}: {}", website, e);
                        }
                    }
                } else {
                    eprintln!("Password format error for {}", website);
                }
            }
            Err(e) => {
                eprintln!("Error retrieving account: {}", e);
            }
        }
    }

    Ok(result)
}

// Setters
#[tauri::command]
pub(crate) fn set_password_hash(new_hash: String) -> Result<(), String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    conn.execute("DELETE FROM master_password", params![])
        .map_err(|e| e.to_string())?;

    conn.execute(
        "INSERT INTO master_password (master_hash) VALUES (?1)",
        params![new_hash],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub(crate) fn insert_user_with_custom_id(email_username: String) -> Result<String, String> {
    // Open the database connection, and convert the error into a String if it fails
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    // Generate the custom ID, and convert the error into a String if it fails
    let id = generate_custom_id(&email_username).map_err(|e| e.to_string())?;

    // Insert the user into the database, and convert the error into a String if it fails
    conn.execute(
        "INSERT INTO users (id, email_username) VALUES (?1, ?2)",
        rusqlite::params![id, email_username],
    )
    .map_err(|e| e.to_string())?;

    // Return a success message if everything went well
    Ok(format!("User inserted with ID: {}", id))
}

#[tauri::command]
pub(crate) fn insert_account_password(user_id: String, website: String, password: String) -> Result<(), String> {
    let db_path = get_database_path();
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;

    let master_password = get_password_hash()?;
    let (salt, nonce, ciphertext) = encrypt_password(master_password, password);

    let encrypted_password = format!("{}|{}|{}", salt, nonce, ciphertext);

    conn.execute(
        "INSERT INTO passwords (user_id, website, password) VALUES (?1, ?2, ?3)",
        params![user_id, website, encrypted_password],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}
