#[path = "cb_gen.rs"]
mod cb_gen;
#[path = "cb_impl.rs"]
pub mod cb_impl;

pub use cb_gen::execute_cb;
