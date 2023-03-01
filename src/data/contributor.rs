use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Contribution {
    Author,
    Unknown(String),
}

impl Contribution {
    pub fn from_string(s: &str) -> Self {
        match s {
            "authors" => Self::Author,
            _ => Self::Unknown(s.to_string()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contributor {
    name: String,
    contribution: Contribution,
}

impl Contributor {
    pub fn new(name: String, contribution: Contribution) -> Self {
        Self {
            name: name,
            contribution: contribution,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn contribution(&self) -> &Contribution {
        &self.contribution
    }
}
