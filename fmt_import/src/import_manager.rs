use crate::import_manager::Import::{MultiLine, SingleLine};

impl ImportProcessor {
    pub fn new() -> Self {
        ImportProcessor {}
    }
}

#[derive(Clone, Default, Debug)]
struct ImportBlock {
    start_idx: usize,
    end_index: usize,
    imports: Vec<Import>,
}

#[derive(Clone, Debug)]
enum Import {
    SingleLine { idx: usize },
    MultiLine { start_idx: usize, end_idx: usize },
}

/// Process imports: groups, sort, etc.
pub struct ImportProcessor {}

impl ImportProcessor {
    pub fn process<'c>(&self, file_content: &'c str) -> Option<&'c str> {
        // todo impl
        Some(file_content)
    }

    /// Fetches all imports from content
    fn find_all_imports(&self, content: &str) -> Vec<ImportBlock> {
        let mut result = vec![];

        let mut inside_import_block = false;
        let mut start_idx_of_multiline_import = None;
        let mut non_import_lines_cnt = 0;

        let mut import_block = ImportBlock::default();

        for (idx, line) in content.lines().enumerate() {
            if line.trim().starts_with("use ") {
                if !inside_import_block {
                    // we reach import block
                    inside_import_block = true;
                    import_block.start_idx = idx;
                }

                if line.trim().ends_with(";") {
                    // single line import
                    import_block.imports.push(SingleLine { idx })
                } else {
                    start_idx_of_multiline_import.replace(idx);
                }
                non_import_lines_cnt = 0;
                continue;
            }

            // end of multiline import
            if line.trim().ends_with(";") && start_idx_of_multiline_import.is_some() {
                let start_idx = start_idx_of_multiline_import.take().unwrap();
                import_block.imports.push(MultiLine {
                    start_idx,
                    end_idx: idx,
                });
                continue;
            }

            if inside_import_block && start_idx_of_multiline_import.is_none() {
                non_import_lines_cnt += 1;
                if non_import_lines_cnt == 2 {
                    // end of import block
                    inside_import_block = false;
                    non_import_lines_cnt = 0;
                    import_block.end_index = idx - 2;
                    result.push(import_block.clone());
                    import_block = ImportBlock::default();
                }
            }
        }

        result
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use insta::*;
    use insta::{assert_debug_snapshot, assert_snapshot};

    #[test]
    fn empty_content() {
        let content = "";
        let processor = ImportProcessor::new();
        assert!(processor.find_all_imports(content).is_empty())
    }

    #[test]
    fn no_import_content() {
        let content = r#"
            pub mod test;
            pub fn main() {
                println!("test");
            }
        "#;
        let processor = ImportProcessor::new();
        assert!(processor.find_all_imports(content).is_empty())
    }

    #[test]
    fn one_block_one_import() {
        let content = r#"
            use crate::import_manager::Import::{MultiLine, SingleLine};
            pub fn main() {
                println!("test");
            }
        "#;
        let processor = ImportProcessor::new();
        assert_debug_snapshot!(processor.find_all_imports(content))
    }

    #[test]
    fn one_block_one_multiline_import() {
        let content = r#"
            use crate::import_manager::Import::{
                MultiLine,
                SingleLine
            };
            pub fn main() {
                println!("test");
            }
        "#;
        let processor = ImportProcessor::new();
        assert_debug_snapshot!(processor.find_all_imports(content))
    }

    #[test]
    fn one_block_many_imports() {
        let content = r#"
            use crate::import_manager::ImportProcessor;
            // todo remove
            use crate::Config;
            use eyre::{Context, Result};
            use ignore::gitignore::{Gitignore, GitignoreBuilder};

            use std::path::{Path, PathBuf};
            use crate::import_manager::Import::{
                MultiLine,
                SingleLine
            };

            use std::ffi::OsStr;
            use ignore::Walk;

            pub fn main() {
                println!("test");
            }
        "#;
        let processor = ImportProcessor::new();
        assert_debug_snapshot!(processor.find_all_imports(content))
    }

    #[test]
    fn three_blocks_many_imports() {
        let content = r#"
            use crate::import_manager::ImportProcessor;
            use crate::Config;
            use eyre::{Context, Result};

            pub mod test {
                use ignore::gitignore::{Gitignore, GitignoreBuilder};

                use std::path::{Path, PathBuf};
                use crate::import_manager::Import::{
                    MultiLine,
                    SingleLine
                };
            }

            pub mod test2 {
                use std::ffi::OsStr;
                use ignore::Walk;

            }
        "#;
        let processor = ImportProcessor::new();
        assert_debug_snapshot!(processor.find_all_imports(content))
    }

}
