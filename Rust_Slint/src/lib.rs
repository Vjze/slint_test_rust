pub mod logic;
pub use logic::*;
#[allow(clippy::all)]
pub mod generated_code {
    slint::include_modules!();
}
pub use generated_code::*;