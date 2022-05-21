#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

// the payload type must implement `Serialize` and `Clone`.
#[derive(Clone, serde::Serialize)]
struct Payload {
  message: String,
}

fn main() {
  let submenu = Submenu::new("Tauri Test", Menu::new()
    .add_native_item(MenuItem::Quit)
  );

  let editmenu = Submenu::new("Edit", Menu::new()
    .add_native_item(MenuItem::Undo)
    .add_native_item(MenuItem::Redo)
    .add_native_item(MenuItem::Separator)
    .add_native_item(MenuItem::Copy)
    .add_native_item(MenuItem::Cut)
    .add_native_item(MenuItem::Paste)
    .add_native_item(MenuItem::Separator)
    .add_native_item(MenuItem::Zoom)
  );

  let testmenu = Submenu::new("Tests", Menu::new()
    .add_item(CustomMenuItem::new("alert".to_string(), "Alert"))
  );

  let menu = Menu::new()
  .add_submenu(submenu)
  .add_submenu(editmenu)
  .add_submenu(testmenu);

  tauri::Builder::default()
    .menu(menu)
    .on_menu_event(|event| {
      let windows = event.window();
      match event.menu_item_id() {
        "quit" => {
          std::process::exit(0);
        }
        "close" => {
          windows.close().unwrap();
        }
        "alert" => {
          windows.emit("backend_alert", Payload { message: "Tauri + Svelte is awesome!".into() }).unwrap();
        }
        _ => {}
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
