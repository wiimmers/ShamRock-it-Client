// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod backup;
mod info;
mod mailer;
mod structures;
mod ninja; 
mod log_to_file;
mod crypto;
mod db;
use crate::backup::backup::{
    update_progress_bar, backup_user
};
use crate::info::info::get_user_frontend;
use crate::mailer::mailer::email_ticket;
use crate::log_to_file::log_to_file::log_builder;
use crate::ninja::ninja::ninja_auth; 
use crate::db::db::ticket_statuses;
use std::env;
use tauri_plugin_log::LogTarget;
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, WindowBuilder};


#[derive(Clone, serde::Serialize)]
struct Payload {
  args: Vec<String>,
  cwd: String,
}

fn main() {

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let submit = CustomMenuItem::new("submit".to_string(), "Submit Tickets");
    let backup = CustomMenuItem::new("backup".to_string(), "Backup User Data");
    let status = CustomMenuItem::new("status".to_string(), "Ticket Statuses");

    let tray_menu = SystemTrayMenu::new()
        .add_item(submit)
        .add_item(backup)
        .add_item(status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit); 

    let tray = SystemTray::new().with_menu(tray_menu).with_tooltip("ShamRock.it | Right Click for Menu");

    let tauri_app = tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick {
                position: _,
                size: _,
                ..
            } => {
                let status_window = app
                    .get_window("ticket_status");
                if status_window != None {
                    status_window
                        .unwrap()
                        .close()
                        .unwrap();
                }
                let ticket_window = app
                    .get_window("submit_ticket");
                if ticket_window != None {
                    ticket_window
                        .unwrap()
                        .close()
                        .unwrap();
                }
                let backup_window = app
                    .get_window("backup_user");
                if backup_window != None {
                    backup_window
                        .unwrap()
                        .close()
                        .unwrap(); 
                }
                let window = app.get_window("main");
                if window == None {
                    WindowBuilder::new(
                        app, 
                        "main",
                        tauri::WindowUrl::App("index.html".into()))
                        .title("ShamRock.it")
                        .build()
                        .expect("Failed to build main window");
                } else {
                    window.unwrap().show().expect("Failed to show window");
                }
            } 
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                    "submit" => {
                        let window = app
                            .get_window("main");
                        if window != None {
                            window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let backup_window = app
                            .get_window("backup_user");

                        if backup_window != None {
                            backup_window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let status_window = app
                            .get_window("ticket_status");
                        if status_window != None {
                            status_window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let ticket_window = app.get_window("submit_ticket"); 
                        if ticket_window == None {
                            WindowBuilder::new(
                                app, 
                                "submit_ticket",
                                tauri::WindowUrl::App("ticket.html".into()))
                                .title("ShamRock.it")
                                .build()
                                .expect("Failed to build submit_ticket window");
                        } else {
                            ticket_window.unwrap().show().expect("Failed to show ticket window");
                        }
                    }
                    "backup" => {
                        let window = app
                            .get_window("main");
                        if window != None {
                            window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let ticket_window = app
                            .get_window("submit_ticket");
                        if ticket_window != None {
                            ticket_window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let status_window = app
                            .get_window("ticket_status");
                        if status_window != None {
                            status_window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let backup_window = app.get_window("backup_user"); 
                        if backup_window == None {
                            WindowBuilder::new(
                                app, 
                                "backup_user",
                                tauri::WindowUrl::App("backup.html".into()))
                                .title("ShamRock.it")
                                .build()
                                .expect("Failed to build submit_ticket window");
                        } else {
                            backup_window.unwrap().show().expect("Failed to show backup window");
                        }
                    }
                    "status" => {
                        let window = app
                            .get_window("main");
                        if window != None {
                            window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let ticket_window = app
                            .get_window("submit_ticket");
                        if ticket_window != None {
                            ticket_window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let backup_window = app
                            .get_window("backup_user");
                        if backup_window != None {
                            backup_window
                                .unwrap()
                                .close()
                                .unwrap();
                        }
                        let status_window = app.get_window("ticket_status"); 
                        if status_window == None {
                            WindowBuilder::new(
                                app, 
                                "ticket_status",
                                tauri::WindowUrl::App("status.html".into()))
                                .title("ShamRock.it")
                                .build()
                                .expect("Failed to build ticket_status window");
                        } else {
                            status_window.unwrap().show().expect("Failed to show backup window");
                        }
                    }
                    "quit" => {
                        std::process::exit(0); 
                    } 
                    _ => {}
                } 
            } _ => {}
        })
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            main_window.hide().expect("error hiding window on startup"); 
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::default()
                .targets([LogTarget::Folder(log_builder())])
                .build(),
        )
        .plugin(
            tauri_plugin_single_instance::init(|app, argv, cwd| {
                println!("{}, {argv:?}, {cwd}", app.package_info().name);
    
                app.emit_all("single-instance", Payload { args: argv, cwd }).unwrap();

                let main_window = app.get_window("main");
                let ticket_window = app.get_window("submit_ticket");
                let backup_window = app.get_window("backup_user");

                if main_window != None {
                    main_window.unwrap().show().expect("Failed to show main window");
                } else if ticket_window != None {
                    ticket_window.unwrap().show().expect("Failed to show ticket window");
                } else if backup_window != None {
                    backup_window.unwrap().show().expect("Failed to show backup window");
                } else {
                    eprintln!("No windows, but instance running, restart the application");
                }

                
            })
        )
        .invoke_handler(tauri::generate_handler![
            backup_user,
            update_progress_bar,
            get_user_frontend,
            email_ticket,
            ninja_auth,
            ticket_statuses
        ])
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    tauri_app.run(|_app_handle, event|
        match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
          api.prevent_exit();
        }
        _ => {}
    });    

}


