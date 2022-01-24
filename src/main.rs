use clap::Parser;
use gdk::{keys, EventKey};
use gtk::prelude::*;
use gtk::Window;
use relm::{connect, Relm, Update, Widget};
use relm_derive::Msg;
use sourceview::{Language, View as SourceView};
use sourceview::{LanguageExt, LanguageManager, LanguageManagerExt};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use tempfile::{tempfile, NamedTempFile, TempPath};

#[derive(Parser)]
pub struct Args {
    /// Sourceview Lang file to watch
    #[clap(short, long)]
    pub lang: String,

    /// Source file to render
    #[clap(short, long)]
    pub test: String,
}

#[derive(Msg)]
pub enum Msg {
    Recompile,
    Quit,
}

pub struct State {
    lang: String,
    test: String,
}

pub struct App {
    compiler: Compiler,
    state: State,
    gui: Widgets,
}

impl Update for App {
    type Model = State;
    type ModelParam = (String, String);
    type Msg = Msg;

    fn model(_: &Relm<Self>, (lang, test): Self::ModelParam) -> State {
        State { lang, test }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Recompile => {
                println!("building");
                let b = self.gui.render_view.get_buffer().unwrap();
                let rendered = b
                    .get_text(&b.get_start_iter(), &b.get_end_iter(), false)
                    .unwrap()
                    .to_string();

                let mut f = File::open(&self.state.test).unwrap();
                let mut lang_src = String::new();
                f.read_to_string(&mut lang_src);

                let b = self.compiler.compile_buffer(&lang_src);
                b.set_text(&rendered);
                self.gui.render_view.set_buffer(Some(&b))
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}

struct Compiler {
    lm: LanguageManager,
    test_file: String,
}

impl Compiler {
    fn new(test_file: String) -> Self {
        let lm = sourceview::LanguageManager::get_default().unwrap();
        Self { lm, test_file }
    }

    fn compile_buffer(&self, txt: &str) -> sourceview::Buffer {
        let file = Path::new("/tmp/langview.lang");
        let mut file = File::create(file).unwrap();
        println!("{}", txt);
        write!(file, "{}", txt);

        let lm = sourceview::LanguageManager::get_default().unwrap();
        let mut sp: Vec<String> = lm.get_search_path().iter().map(|s| s.to_string()).collect();
        sp.push("/tmp".into());

        let lm = sourceview::LanguageManagerBuilder::new()
            .search_path(sp.into())
            .build();

        let test_lang = lm.guess_language(Some(&self.test_file), None).unwrap();
        sourceview::Buffer::new_with_language(&test_lang)
    }
}

impl Widget for App {
    type Root = Window;

    fn init_view(&mut self) {
        let mut f = File::open(&self.state.lang).unwrap();
        let mut lang_src = String::new();
        f.read_to_string(&mut lang_src);

        let mut f = File::open(&self.state.test).unwrap();
        let mut test_txt = String::new();
        f.read_to_string(&mut test_txt);

        let buffer = self.compiler.compile_buffer(&lang_src);
        buffer.set_text(&test_txt);
        self.gui.render_view.set_buffer(Some(&buffer));
    }

    fn root(&self) -> Self::Root {
        self.gui.main_window.clone()
    }

    fn view(relm: &Relm<Self>, state: Self::Model) -> Self {
        let glade_src = include_str!("ui.glade");
        let builder = gtk::Builder::from_string(glade_src);
        let main_window: gtk::Window = builder.get_object("main_window").unwrap();
        let render_view: sourceview::View = builder.get_object("render_view").unwrap();

        connect!(
            relm,
            main_window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        main_window.show_all();

        App {
            compiler: Compiler::new(state.test.clone()),
            state,
            gui: Widgets {
                render_view,
                main_window,
            },
        }
    }
}

struct Widgets {
    render_view: SourceView,
    main_window: Window,
}

fn main() {
    let args: Args = Args::parse();
    App::run((args.lang.clone(), args.test.clone())).expect("App::run failed");
}
