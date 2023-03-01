use std::path::Iter;

use serde::{Deserialize, Serialize};

use super::{contributor::Contributor, file::Files};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Book {
    title: String,
    contributors: Vec<Contributor>,
    isbn: Option<String>,
    files: Option<Files>,
}

impl Book {
    pub fn new<I: Iterator<Item = Contributor>>(title: String, contributors: I) -> Self {
        Self {
            title: title,
            contributors: Vec::from_iter(contributors),
            isbn: None,
            files: None,
        }
    }

    pub fn with_files<I: Iterator<Item = Contributor>>(
        title: String,
        contributors: I,
        files: Option<Files>,
    ) -> Self {
        Self {
            title: title,
            contributors: Vec::from_iter(contributors),
            isbn: None,
            files: files,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn contributors(&self) -> &Vec<Contributor> {
        &self.contributors
    }

    pub fn isbn(&self) -> Option<&str> {
        match &self.isbn {
            Some(isbn) => Some(isbn),
            None => None,
        }
    }

    pub fn files(&self) -> Option<&Files> {
        match &self.files {
            Some(files) => Some(files),
            None => None,
        }
    }

    pub fn set_isbn(&mut self, isbn: String) {
        self.isbn = Some(isbn);
    }

    pub fn set_files(&mut self, files: Files) {
        self.files = Some(files);
    }
}
