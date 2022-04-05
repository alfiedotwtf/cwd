use std::env::{current_dir, var};

pub fn cwd() -> String {
    if let Ok(current_dir) = current_dir() {
        if let Some(current_dir_str) = current_dir.to_str() {
            return current_dir_str.to_string();
        }
    }

    if let Ok(pwd) = var("PWD") {
        return pwd;
    }

    String::from('.')
}
