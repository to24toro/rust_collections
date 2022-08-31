#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

fn die(e: std::io::Error) {
    panic!("{}", e);
}

fn main() {
    Editor::default().run();
}
