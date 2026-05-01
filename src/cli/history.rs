use std::fs;
use std::path::PathBuf;

pub fn get_history_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".shell_history")
}

pub fn init_history() -> std::io::Result<()> {
    let history_path = get_history_path();
    if !history_path.exists() {
        fs::File::create(history_path)?;
    }
    Ok(())
}

pub fn get_history_path_string() -> String {
    get_history_path().to_string_lossy().to_string()
}
