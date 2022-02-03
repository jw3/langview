use relaxng_model::{model, Compiler as NgCompiler, FsFiles, Syntax};
use relaxng_validator::Validator as NgValidator;
use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::exit;
use std::rc::Rc;

pub(crate) type RelaxNgModelRef = Rc<RefCell<Option<model::DefineRule>>>;

pub struct Validator {
    model: RelaxNgModelRef,
}

impl Validator {
    pub fn new(schema: PathBuf) -> Self {
        let mut compiler = NgCompiler::new(FsFiles, Syntax::Xml);
        let model = match compiler.compile(&schema) {
            Ok(m) => m,
            Err(err) => {
                compiler.dump_diagnostic(&err);
                exit(1);
            }
        };

        Self { model }
    }

    pub fn validate(&self, xml: PathBuf) {
        let mut f = File::open(&xml).expect("open example xml");
        let mut doc = String::new();
        f.read_to_string(&mut doc).expect("read xml");
        let src = doc.clone();
        let reader = xmlparser::Tokenizer::from(&src[..]);
        let mut v = NgValidator::new(self.model.clone(), reader);
        eprintln!("Validating {:?}", xml);
        loop {
            match v.next() {
                Some(Ok(())) => {}
                Some(Err(err)) => {
                    let (map, d) = v.diagnostic(xml.to_string_lossy().to_string(), doc, &err);
                    let mut emitter = codemap_diagnostic::Emitter::stderr(
                        codemap_diagnostic::ColorConfig::Auto,
                        Some(&map),
                    );
                    emitter.emit(&d[..]);
                    break;
                }
                None => break,
            }
        }
    }
}
