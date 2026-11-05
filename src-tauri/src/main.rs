#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn main() {
    std::panic::set_hook(Box::new(|panic| {
        let details = panic.to_string();
        let diagnostic_saved = report_backend_startup_error(&details);
        show_backend_startup_error(diagnostic_saved);
    }));

    if let Err(error) = life_planner_lib::run() {
        let diagnostic_saved = report_backend_startup_error(&error.to_string());
        show_backend_startup_error(diagnostic_saved);
    }
}

fn report_backend_startup_error(details: &str) -> bool {
    let entry = format!(
        "{} [backend-bootstrap] {details}\n",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    );

    let log_directory = application_state_directory().join("logs");
    let result = fs::create_dir_all(&log_directory).and_then(|()| {
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_directory.join("startup.log"))?
            .write_all(entry.as_bytes())
    });

    match result {
        Ok(()) => true,
        Err(log_error) => {
            eprintln!("[Life Planner][backend-bootstrap] {details}");
            eprintln!(
                "[Life Planner][diagnostics] Não foi possível gravar startup.log: {log_error}"
            );
            false
        }
    }
}

fn application_state_directory() -> PathBuf {
    #[cfg(target_os = "windows")]
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .unwrap_or_else(std::env::temp_dir);

    #[cfg(target_os = "linux")]
    let base = std::env::var_os("XDG_STATE_HOME")
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .map(PathBuf::from)
                .map(|home| home.join(".local").join("state"))
        })
        .unwrap_or_else(std::env::temp_dir);

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    let base = std::env::temp_dir();

    base.join("Life Planner")
}

#[cfg(target_os = "windows")]
fn show_backend_startup_error(diagnostic_saved: bool) {
    use std::{iter, os::windows::ffi::OsStrExt, ptr};

    #[link(name = "user32")]
    unsafe extern "system" {
        #[link_name = "MessageBoxW"]
        fn message_box_w(
            window: *mut std::ffi::c_void,
            text: *const u16,
            caption: *const u16,
            kind: u32,
        ) -> i32;
    }

    let message_text = if diagnostic_saved {
        "O Life Planner encontrou um erro ao iniciar. O diagnóstico técnico foi registrado localmente."
    } else {
        "O Life Planner encontrou um erro ao iniciar. Não foi possível registrar o diagnóstico técnico."
    };
    let message: Vec<u16> = std::ffi::OsStr::new(message_text)
        .encode_wide()
        .chain(iter::once(0))
        .collect();
    let caption: Vec<u16> = std::ffi::OsStr::new("Life Planner")
        .encode_wide()
        .chain(iter::once(0))
        .collect();

    // The message and caption are static UTF-16 strings owned for the duration of this call.
    unsafe {
        message_box_w(
            ptr::null_mut(),
            message.as_ptr(),
            caption.as_ptr(),
            0x0000_0010,
        );
    }
}

#[cfg(not(target_os = "windows"))]
fn show_backend_startup_error(_diagnostic_saved: bool) {
    eprintln!("[Life Planner][backend-bootstrap] O aplicativo não conseguiu iniciar.");
}
