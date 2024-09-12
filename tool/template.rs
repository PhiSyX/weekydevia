use std::ffi::OsString;
use std::io::{self};
use std::path::{self, PathBuf};
use std::sync::Arc;

use tokio::task;

use crate::chan::{TemplateSender, TemplateState};
use crate::error::Error;

// ---- //
// Type //
// ---- //

pub type Result<T> = std::result::Result<T, Error>;

// --------- //
// Structure //
// --------- //

pub struct Template {
    root_directory: PathBuf,
    content: String,
    tx: Option<TemplateSender>,
}

// -------------- //
// Implémentation //
// -------------- //

impl Template {
    pub fn new(root_directory: path::PathBuf, content: String) -> Self {
        Self {
            root_directory,
            content,
            tx: Default::default(),
        }
    }

    pub fn open(file: impl AsRef<path::Path> + ToOwned<Owned = path::PathBuf>) -> io::Result<Self> {
        let template = std::fs::read_to_string(&file)?;
        let mut file = file.to_owned();
        file.pop();
        Ok(Self::new(file, template))
    }

    pub fn shared(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn with_sender(mut self, sender: &TemplateSender) -> Self {
        self.tx.replace(sender.to_owned());
        self
    }
}

impl Template {
    fn tx(&self) -> &TemplateSender {
        assert!(self.tx.is_some());
        self.tx.as_ref().unwrap()
    }

    fn parse_meta_include(
        &self,
        line: &str,
        prev_filepath: impl AsRef<path::Path> + Send + 'static,
    ) -> Option<task::JoinHandle<Result<()>>> {
        if !(line.starts_with("#include <") && line.ends_with('>')) {
            return None;
        }

        // EXAMPLE: "#include <dir/file.md>"
        //
        // NOTE(phisyx): on ne veut garder que le chemin, à savoir :
        // `dir/file.md`.
        let relative_filepath = self
            .root_directory
            .join(line.replace("#include <", "").replace('>', ""));

        if !relative_filepath.is_file() {
            eprintln!(
                "WARN: il ne s'agit pas d'un fichier valide: {}",
                relative_filepath.display()
            );
            return None;
        }

        // SAFETY(unwrap): nous savons que le fichier existe grâce à la
        // condition ci-haut.
        let template = Self::open(relative_filepath.to_owned())
            .ok()?
            .with_sender(self.tx())
            .shared();

        Some(template.spawn(prev_filepath, relative_filepath))
    }
}

impl Template {
    pub fn spawn(
        self: Arc<Self>,
        prev_filepath: impl AsRef<path::Path> + Send + 'static,
        relative_filepath: impl AsRef<path::Path> + Clone + Send + 'static,
    ) -> task::JoinHandle<Result<()>> {
        tokio::spawn(self.process(prev_filepath, relative_filepath))
    }

    pub async fn process(
        self: Arc<Self>,
        prev_filepath: impl AsRef<path::Path> + Send,
        output_filepath: impl AsRef<path::Path> + Clone + Send + 'static,
    ) -> Result<()> {
        let prev_filename = prev_filepath.as_ref().file_name().unwrap();
        let output_filename = output_filepath.as_ref().file_name().unwrap();

        for line in self.content.lines() {
            if let Some(handle) = self.parse_meta_include(line, output_filepath.clone()) {
                handle.await.unwrap()?;
            } else {
                if line == "--- THE END ---" {
                    _ = self.tx().send(TemplateState::EOF);
                    break;
                }

                let mut line = line.to_owned();

                let escape_md_link = |s: &str| s.replace("_", "&#95;");

                if line.trim_start().starts_with("http") {
                    line = format!("[{0}]({0})", escape_md_link(&line).trim());
                } else if line.trim_start_matches("1. ").starts_with("http")
                    || line.trim_start_matches("2. ").starts_with("http")
                    || line.trim_start_matches("3. ").starts_with("http")
                    || line.trim_start_matches("4. ").starts_with("http")
                    || line.trim_start_matches("5. ").starts_with("http")
                    || line.trim_start_matches("6. ").starts_with("http")
                    || line.trim_start_matches("7. ").starts_with("http")
                    || line.trim_start_matches("8. ").starts_with("http")
                    || line.trim_start_matches("9. ").starts_with("http")
                    || line.trim_start_matches("10. ").starts_with("http")
                {
                    line = format!(
                        "{0} [{1}]({1})",
                        &line[0..2],
                        escape_md_link(&line[3..]).trim()
                    );
                }

                let filename: OsString = if output_filename.to_string_lossy().starts_with('_') {
                    prev_filename
                } else {
                    output_filename
                }
                .to_owned();

                _ = self.tx().send(TemplateState::Content {
                    filename,
                    text: format!("{line}\r\n"),
                });
            }
        }

        Ok(())
    }
}

// -------- //
// Fonction //
// -------- //

#[inline]
pub fn replace_relative_links(rel: impl std::fmt::Display, input: &str) -> String {
    input.replace("](./", &format!("]({rel}"))
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

unsafe impl Sync for Template {}
unsafe impl Send for Template {}
