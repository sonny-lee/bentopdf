#[tauri::command]
fn get_system_locale() -> String {
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleLocale"])
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let trimmed = s.trim().to_string();
                if !trimmed.is_empty() {
                    return trimmed;
                }
            }
        }
        if let Ok(output) = std::process::Command::new("defaults")
            .args(["read", "-g", "AppleLanguages"])
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                for line in s.lines() {
                    let cleaned = line
                        .trim()
                        .trim_matches(|c| c == '"' || c == '(' || c == ')' || c == ',');
                    if !cleaned.is_empty() && !cleaned.contains("AppleLanguages") {
                        return cleaned.to_string();
                    }
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "[System.Globalization.CultureInfo]::CurrentUICulture.Name",
            ])
            .output()
        {
            if let Ok(s) = String::from_utf8(output.stdout) {
                let trimmed = s.trim().to_string();
                if !trimmed.is_empty() {
                    return trimmed;
                }
            }
        }
    }

    std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LC_MESSAGES"))
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_else(|_| "en".to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_system_locale])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
