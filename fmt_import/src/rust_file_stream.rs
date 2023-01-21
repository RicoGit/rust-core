//! Walks file tree, filters files and folder defined in config and provides Rust files content.

use std::ffi::OsStr;
use eyre::{Context, Result};
use std::path::{Path, PathBuf};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore::Walk;
use crate::Config;

/// Recursively traverse files and find all Rust files
pub struct FileProcessor {
    root: PathBuf,
    gitignore: Gitignore,
    import_processor: ImportProcessor
}

impl FileProcessor {
    pub fn new(root: PathBuf, config: Config, import_processor: ImportProcessor) -> Result<Self> {
        let mut builder = GitignoreBuilder::new(root.as_path());
        for ignore in config.ignore.into_iter() {
            builder.add_line(None, &ignore).with_context(|| "Wrong ignore glob (use .gitignore format)")?;
        }
        let gitignore = builder.build()?;

        Ok(Self {
            root,
            gitignore,
            import_processor
        })
    }

    pub fn start(self) {
        let RS = OsStr::new("rs");
        for entry in Walk::new(self.root)
            .into_iter()
            .filter_map(|e| e.ok()) // ignore errors
            .filter(|dir| {
                // filter ignored folder/files from config
                let path = dir.path();
                !self.gitignore.matched(&path, path.is_dir()).is_ignore()
            }) {
            let path = entry.path();
            if path.extension().filter(|ext| *ext == RS).is_some() {
                // todo get content of Rust files
                println!("{}", path.display());
            }
        }
    }

}

/// Process imports: groups, sort, etc.
pub struct ImportProcessor {

}

impl ImportProcessor {

    pub fn process<'content, 'processor: 'content>(&'processor self, lines: &'content [&'content str]) -> &'content [&'content str] {
        // noop implementation 
        lines
    }

}


