use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Files {
    main: PathBuf,
}

impl Files {
    pub fn with_main(main: PathBuf) -> Self {
        Self { main }
    }

    pub fn main(&self) -> &Path {
        &self.main
    }
}
