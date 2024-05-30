use std::ffi::OsString;
use std::io::{self};
use std::path::{self, PathBuf};
use std::sync::Arc;

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
    ) -> Option<tokio::task::JoinHandle<std::result::Result<(), Error>>> {
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
            return None;
        }

        // SAFETY(unwrap): nous savons que le fichier existe grâce à la
        // condition ci-haut.
        let template = Self::open(relative_filepath.to_owned())
            .unwrap()
            .with_sender(self.tx());

        let shared_template = Arc::new(template);
        let handle = tokio::spawn(shared_template.process(prev_filepath, relative_filepath));

        Some(handle)
    }
}

impl Template {
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
                if line.starts_with("http") {
                    line = format!("[{0}]({0})", line.trim_end());
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
