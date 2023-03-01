use cursive::{Cursive, views::Dialog};

use crate::data::Book;

#[derive(Debug, Clone, Copy)]
pub struct Viewer;

impl Viewer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn show_main() {}

    /// Open the given book.
    pub fn open_book(s: &mut Cursive, book: &Book) {
        if let Some(files) = book.files() {
            let files = files.clone();
            open::that(files.main()).unwrap();
        }
    }
}
