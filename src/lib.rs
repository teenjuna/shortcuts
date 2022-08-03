//! **Shortcuts** for popular methods of popular types that allow you to write
//! your **blazingly fast** code **blazingly fast**.
//!
//! ```
//! use shortcuts::prelude::*;
//!
//! let v = Some("blazingly fast");
//!
//! v.unwrap(); // too long to type
//! v.un();     // perfection
//! ```

/// Shortcuts for [core::option].
pub mod option;
/// All the shortcuts you need in one place!
pub mod prelude;
/// Shortcuts for [core::result].
pub mod result;
