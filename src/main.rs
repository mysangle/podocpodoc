
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::print_stdout,
    clippy::arithmetic_side_effects,
    clippy::as_conversions,
    clippy::integer_division
)]
use podocpodoc::editor::Editor;

fn main() {
    Editor::new().unwrap().run();
}

