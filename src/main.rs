mod data;
mod viewer;

use std::{borrow::Cow, path::PathBuf};

use clap::Parser;
use cursive::{
    traits::{Nameable, Scrollable},
    views::{Dialog, LinearLayout, ResizedView, SelectView, TextView},
    Cursive,
};
use data::{Contribution, Contributor, Files};
use rusqlite::Connection;
use viewer::Viewer;

use crate::data::Book;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(default_value_t = String::from("books.db"))]
    db: String,
}


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let con = Connection::open(args.db)?;
    let mut siv = cursive::default();

    let mut stmt = con.prepare(
        "SELECT books.id, books.title, files.path FROM books OUTER LEFT JOIN files ON files.work = books.id GROUP BY books.id ORDER BY books.title;",
    )?;

    let books = stmt.query_map([], |row| {
        let mut stmt = con.prepare(
            "SELECT contributors.name, contributions.type FROM contributors JOIN contributions WHERE contributions.contributor = contributors.id AND contributions.work = ? GROUP BY contributors.id ORDER BY contributors.name;"
        )?;
        let book_id: u8 = row.get(0)?;
        let contributors = stmt.query_map([book_id], |r| {
            let name: String = r.get(0)?;
            let contribution: String = r.get(1)?;
            Ok(Contributor::new(name, Contribution::from_string(&contribution)))
        }).expect("Could not create contributors list.");
        
        Ok(Book::with_files(row.get(1)?, contributors.map(Result::unwrap), row.get(2).ok().map(|p: String| Files::with_main(PathBuf::from(p)))))
    })?;

    let mut book_list = SelectView::<Book>::new()
        .on_submit(Viewer::open_book)
        .on_select(|s, book| {
            let mut book_info = s.find_name::<TextView>("book_info").unwrap();
            let file_path = if let Some(files) = book.files() {
                files.main().to_string_lossy()
            } else {
                Cow::from("No File")
            };
            book_info.set_content(format!(
                "# {}\nby {}\n\t{}",
                book.title(),
                book.contributors()
                    .iter()
                    .map(|c| c.name())
                    .collect::<Vec<&str>>()
                    .join(" & "),
                file_path
            ));
        });
    book_list.add_all(books.map(|r| match r {
        Ok(book) => (book.title().to_string(), book),
        Err(e) => panic!("{}", e),
    }));

    let book_info = TextView::new("Select a book.").with_name("book_info");

    let lr = ResizedView::with_full_screen(
        LinearLayout::horizontal()
            .child(book_list.scrollable())
            .child(book_info),
    );

    siv.add_layer(Dialog::around(lr).title("Recall Knowledge"));

    siv.run();

    Ok(())
}
