use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::time::Duration;
use tokio::{spawn, time::sleep};


mod auth;
mod data;

fn close_windows(app: &AppHandle) {
    // Attempt to close the "password_window"
    if let Some(password_window) = app.get_webview_window("password_window") {
        if let Err(e) = password_window.close() {
            eprintln!("Failed to close password window: {}", e);
        }
    } else {
        println!("Password window not found or already closed.");
    }

    // Close the "start" window
    if let Some(start_window) = app.get_webview_window("start_window") {
        if let Err(e) = start_window.close() {
            eprintln!("Failed to close start window: {}", e);
        }
    } else {
        eprintln!("Error: Start window not found.");
    }
}

#[tauri::command]
fn sign_out(app: AppHandle) {
    if app.get_webview_window("start_window").is_none() {
        if let Err(e) = WebviewWindowBuilder::new(
            &app,
            "start_window", // Label must match the original
            WebviewUrl::App("index.html".into()) // Point to the original content
        )
        .title("Password Manager")
        .inner_size(650.0, 450.0)
        .resizable(false)
        .fullscreen(false)
        .build()
        {
            eprintln!("Failed to reopen start window: {}", e);
        } else {
            println!("Start window reopened successfully.");
        }
    } else {
        println!("Start window is already open.");
    }

    if let Some(main_window) = app.get_webview_window("main_window") {
        if let Err(e) = main_window.close() {
            eprintln!("Failed to close main window: {}", e);
        }
    } else {
        println!("Main window not found or already closed.");
    }
}

#[tauri::command]
fn create_app_window(app: AppHandle) {
    // Create the main window
    WebviewWindowBuilder::new(&app, "main_window", WebviewUrl::App("main.html".into()))
        .title("Password Manager")
        .inner_size(1200.0, 800.0)
        .resizable(false)
        .build()
        .expect("Failed to create main window");

    close_windows(&app);
}

#[tauri::command]
fn create_password_window(app: AppHandle) {
    WebviewWindowBuilder::new(
        &app,
        "password_window",
        WebviewUrl::App("password.html".into()),
    )
    .title("Create Password")
    .inner_size(450.0, 400.0)
    .resizable(false)
    .build()
    .expect("Failed to create password window");
}

#[tauri::command]
fn is_password_open(app: AppHandle) -> Result<bool, bool> {
    if let Some(_password_window) = app.get_webview_window("password_window") {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tauri::command]
async fn copy_to_clipboard(text: String) -> Result<(), String> {
    let mut ctx = ClipboardContext::new().map_err(|e| e.to_string())?;
    ctx.set_contents(text).map_err(|e| e.to_string())?;

    // Spawn a background task to clear the clipboard after 10 seconds
    spawn(async move {
        sleep(Duration::from_secs(10)).await;
        if let Ok(mut ctx) = ClipboardContext::new() {
            let _ = ctx.set_contents("".to_string());
        }
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    if let Err(e) = data::create_database() {
        eprintln!("Error creating database: {}", e);
        return;
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            create_password_window,
            create_app_window,
            is_password_open,
            copy_to_clipboard,
            sign_out,
            data::get_password_hash,
            data::set_password_hash,
            data::insert_user_with_custom_id,
            data::get_user_data,
            data::delete_account,
            data::delete_password,
            data::insert_account_password,
            data::get_account_passwords,
            auth::hash_password,
            auth::verify_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
