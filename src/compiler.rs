use std::fs::File;
use std::io::Write;
use std::path::Path;

use sourceview::{LanguageManager, LanguageManagerExt};

pub(crate) struct Compiler {
    lm: LanguageManager,
    test_file: String,
}

impl Compiler {
    pub(crate) fn new(test_file: String) -> Self {
        let lm = sourceview::LanguageManager::get_default().unwrap();
        let mut sp: Vec<String> = lm.get_search_path().iter().map(|s| s.to_string()).collect();
        sp.push("/tmp".into());
        let lm = sourceview::LanguageManagerBuilder::new()
            .search_path(sp.into())
            .build();

        Self { lm, test_file }
    }

    pub fn compile_buffer(&self, txt: &str) -> sourceview::Buffer {
        let file = Path::new("/tmp/langview.lang");
        let mut file = File::create(file).unwrap();
        write!(file, "{}", txt);

        let test_lang = self.lm.guess_language(Some(&self.test_file), None).unwrap();
        sourceview::Buffer::new_with_language(&test_lang)
    }
}
