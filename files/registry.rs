//! Windows registry read helpers for Milanote Desktop.

pub const APP_KEY: &str = "Software\\Milanote\\Milanote Desktop";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Milanote Desktop";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Milanote Desktop") },
        RegValue { name: "Version",     value: String::from("v4.6.9") },
        RegValue { name: "Channel",     value: String::from("lts") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Milanote Desktop") },
        RegValue { name: "Publisher",       value: String::from("Milanote") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Milanote Desktop\\uninstall.exe\"") },
    ]
}
