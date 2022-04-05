//! cwd - Current Working Directory, a convenience crate
//!
//! Instead of copy-pasting this bit of code each time I need the fully qualified current working
//! directory, I decided to turn it into a convenience crate...
//!
//! ## Example
//!
//! ```
//! use cwd::cwd;
//!
//! fn main() {
//!     println!("The current working directory is '{}'", cwd());
//! }
//! ```

use std::env::{current_dir, var};

/// Get the current working directory
///
/// # Parameters
///
/// None
///
/// # Returns
///
/// `String` is the current working directory.
///
/// *Note* This is best effort. If it can't find the fully qualified current working directory for
/// some reason, it will fallback to "."
///
/// # Example
///
/// ```
/// use cwd::cwd;
///
/// fn main() {
///     println!("The current working directory is '{}'", cwd());
/// }
/// ```
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
