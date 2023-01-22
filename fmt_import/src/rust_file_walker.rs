//! Walks file tree, filters files and folder defined in config and provides Rust files content.

use crate::import_manager::ImportProcessor;
use crate::Config;
use eyre::{Context, Result};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Walk;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// Recursively traverse files and find all Rust files
pub struct FileProcessor {
    root: PathBuf,
    gitignore: Gitignore,
    import_processor: ImportProcessor,
}

impl FileProcessor {
    pub fn new(root: PathBuf, config: Config, import_processor: ImportProcessor) -> Result<Self> {
        let mut builder = GitignoreBuilder::new(root.as_path());
        for ignore in config.ignore.into_iter() {
            builder
                .add_line(None, &ignore)
                .with_context(|| "Wrong ignore glob (use .gitignore format)")?;
        }
        let gitignore = builder.build()?;

        Ok(Self {
            root,
            gitignore,
            import_processor,
        })
    }

    pub fn start(self) -> Result<usize> {
        let mut affected_files = 0;
        for entry in Walk::new(self.root)
            .into_iter()
            .filter_map(|e| e.ok()) // ignore errors
            .filter(|dir| {
                let path = dir.path();
                !self.gitignore.matched(&path, path.is_dir()).is_ignore()
            })
        {
            let path = entry.path();
            let rust_file_extension = OsStr::new("rs");
            if path
                .extension()
                .filter(|ext| *ext == rust_file_extension)
                .is_some()
            {
                let content = std::fs::read_to_string(path)?;
                if let Some(updated) = self.import_processor.process(&content) {
                    std::fs::write(path, updated)?;
                    affected_files += 1;
                }
            }
        }

        Ok(affected_files)
    }
}
