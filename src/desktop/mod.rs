//! Desktop shell: CLI, packaging, panic logging, ROM discovery, and Tauri host API.
//! Compiled only with the `pocketemulator` binary — not part of the library crate surface.

pub mod cli;
pub mod logging;
pub mod package;
pub mod panic_hook;
pub mod roms;
pub mod tauri_api;
