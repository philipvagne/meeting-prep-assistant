//! Purpose: host desktop validation behavior for the spike application.
//! Responsibilities: tray setup, background window handling, autostart plugin registration,
//! and Spike 3 Google Workspace validation command wiring.
//! Inputs: Tauri lifecycle events, tray interactions, and frontend command invocations.
//! Outputs: a running validation shell with Google auth, Calendar retrieval,
//! and Gmail / Drive context-collection hooks.
//! Non-responsibilities: AI generation, meeting logic, or final product UX.

mod google_auth;

use google_auth::{
    collect_meeting_context_impl, connect_google_impl, disconnect_google_impl,
    fetch_upcoming_calendar_events_impl, get_google_auth_status_impl,
    save_google_client_config_impl, CalendarEventSummary, GoogleAuthStatus,
    MeetingContextCollection,
};
use serde::Serialize;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager, WindowEvent,
};

#[derive(Serialize)]
struct WindowState {
    visible: bool,
    focused: bool,
    minimized: bool,
}

fn main_window(app: &AppHandle) -> Result<tauri::WebviewWindow, String> {
    app.get_webview_window("main")
        .ok_or_else(|| "main window not found".to_string())
}

fn read_window_state(window: &tauri::WebviewWindow) -> WindowState {
    WindowState {
        visible: window.is_visible().unwrap_or(false),
        focused: window.is_focused().unwrap_or(false),
        minimized: window.is_minimized().unwrap_or(false),
    }
}

fn set_window_hidden_from_taskbar(window: &tauri::WebviewWindow, hidden: bool) {
    #[cfg(target_os = "windows")]
    {
        let _ = window.set_skip_taskbar(hidden);
    }

    #[cfg(not(target_os = "windows"))]
    let _ = (window, hidden);
}

fn restore_main_window_impl(app: &AppHandle) -> Result<WindowState, String> {
    let window = main_window(app)?;

    set_window_hidden_from_taskbar(&window, false);
    let _ = window.show();
    let _ = window.unminimize();
    let _ = window.set_focus();

    Ok(read_window_state(&window))
}

fn hide_main_window_impl(app: &AppHandle) -> Result<WindowState, String> {
    let window = main_window(app)?;

    let _ = window.unminimize();
    set_window_hidden_from_taskbar(&window, true);
    let _ = window.hide();

    Ok(read_window_state(&window))
}

#[tauri::command]
fn restore_main_window(app: AppHandle) -> Result<WindowState, String> {
    restore_main_window_impl(&app)
}

#[tauri::command]
fn hide_main_window(app: AppHandle) -> Result<WindowState, String> {
    hide_main_window_impl(&app)
}

#[tauri::command]
fn get_main_window_state(app: AppHandle) -> Result<WindowState, String> {
    let window = main_window(&app)?;
    Ok(read_window_state(&window))
}

#[tauri::command]
fn get_google_auth_status(app: AppHandle) -> Result<GoogleAuthStatus, String> {
    get_google_auth_status_impl(&app)
}

#[tauri::command]
fn save_google_client_config(
    app: AppHandle,
    client_id: String,
    client_secret: String,
) -> Result<GoogleAuthStatus, String> {
    save_google_client_config_impl(&app, client_id, client_secret)
}

#[tauri::command]
async fn connect_google(app: AppHandle) -> Result<GoogleAuthStatus, String> {
    connect_google_impl(&app).await
}

#[tauri::command]
async fn fetch_upcoming_calendar_events(
    app: AppHandle,
) -> Result<Vec<CalendarEventSummary>, String> {
    fetch_upcoming_calendar_events_impl(&app).await
}

#[tauri::command]
async fn collect_meeting_context(
    app: AppHandle,
    event_id: String,
) -> Result<MeetingContextCollection, String> {
    collect_meeting_context_impl(&app, event_id).await
}

#[tauri::command]
fn disconnect_google(app: AppHandle) -> Result<(), String> {
    disconnect_google_impl(&app)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let open_item =
                MenuItem::with_id(app, "open", "Open Meeting Prep Assistant", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_item, &quit_item])?;
            let app_handle = app.handle().clone();

            TrayIconBuilder::with_id("main-tray")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .tooltip("Meeting Prep Assistant")
                .icon(app.default_window_icon().unwrap().clone())
                .on_tray_icon_event(move |_tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let _ = restore_main_window_impl(&app_handle);
                    }
                })
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => {
                        let _ = restore_main_window_impl(app);
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }

            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = hide_main_window_impl(&window.app_handle());
            }
        })
        .invoke_handler(tauri::generate_handler![
            restore_main_window,
            hide_main_window,
            get_main_window_state,
            get_google_auth_status,
            save_google_client_config,
            connect_google,
            fetch_upcoming_calendar_events,
            collect_meeting_context,
            disconnect_google
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
