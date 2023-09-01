#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
mod communicate;
pub use communicate::{query_login};