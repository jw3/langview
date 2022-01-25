use std::fs::File;
use std::io::Write;
use std::path::Path;

use sourceview::{LanguageManager, LanguageManagerExt};
use tempfile::NamedTempFile;

pub(crate) struct Compiler {
    test_file: String,
    lang_search_paths: Vec<String>,
}

impl Compiler {
    pub(crate) fn new(test_file: String) -> Self {
        let mut lang_search_paths: Vec<String> = LanguageManager::get_default()
            .unwrap()
            .get_search_path()
            .iter()
            .map(|s| s.to_string())
            .collect();

        // todo;; cli-opts to customize the temp dir
        lang_search_paths.push("/tmp".into());

        Self {
            lang_search_paths,
            test_file,
        }
    }

    pub fn compile_buffer(&self, txt: &str) -> sourceview::Buffer {
        let lm = sourceview::LanguageManagerBuilder::new()
            .search_path(self.lang_search_paths.clone())
            .build();

        // todo;; cli-opts to customize and persist the temp file
        let file = tempfile::Builder::new().suffix(".lang").tempfile().unwrap();
        write!(file.as_file(), "{}", txt);

        let test_lang = lm.guess_language(Some(&self.test_file), None).unwrap();
        sourceview::Buffer::new_with_language(&test_lang)
    }
}
