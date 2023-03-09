#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      #[cfg(target_os = "macos")] {
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
          .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
        set_shadow(&window, true)
          .expect("error while window_shadows::set_shadow");
      }

      #[cfg(target_os = "windows")] {
        apply_blur(&window, Some((18, 18, 18, 125)))
          .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
        set_shadow(&window, true)
          .expect("error while window_shadows::set_shadow");
      }

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
