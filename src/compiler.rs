use std::io::Write;
use std::path::PathBuf;

use crate::validation::Validator;
use sourceview::{LanguageManager, LanguageManagerExt};

pub(crate) struct Compiler {
    test_file: String,
    lang_search_paths: Vec<String>,
    validator: Option<Validator>,
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

        let validator = lang_search_paths
            .iter()
            .map(|p| PathBuf::from(p).join("language2.rng"))
            .find(|p| p.exists())
            .map(|p| Validator::new(PathBuf::from(p)));

        match validator {
            None => println!("failed to find language rng, validation is disabled"),
            Some(_) => println!("found langauage rng, validation enabled"),
        }

        Self {
            lang_search_paths,
            test_file,
            validator,
        }
    }

    pub fn compile_buffer(&self, txt: &str) -> sourceview::Buffer {
        let lm = sourceview::LanguageManagerBuilder::new()
            .search_path(self.lang_search_paths.clone())
            .build();

        // todo;; cli-opts to customize and persist the temp file
        let file = tempfile::Builder::new().suffix(".lang").tempfile().unwrap();
        write!(file.as_file(), "{}", txt);

        self.validator
            .iter()
            .for_each(|v| v.validate(PathBuf::from(file.path())));

        let test_lang = lm.guess_language(Some(&self.test_file), None).unwrap();
        sourceview::Buffer::new_with_language(&test_lang)
    }
}
