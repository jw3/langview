use std::fs::File;
use std::io::Read;
use std::thread;
use std::time::Duration;

use gtk::prelude::*;
use gtk::Window;
use relm::{connect, Channel, Relm, Update, Widget};
use relm_derive::Msg;
use sourceview::View as SourceView;

use crate::compiler::Compiler;
use crate::watch::NotifyReceiver;

pub struct Langview {
    compiler: Compiler,
    state: State,
    gui: Widgets,
}

pub struct State {
    lang: String,
    test: String,
    channel: Channel<i32>,
}

struct Widgets {
    render_view: SourceView,
    main_window: Window,
}

#[derive(Msg)]
pub enum Msg {
    Recompile,
    Quit,
}

impl Widget for Langview {
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
        let glade_src = include_str!("gui.glade");
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

        Langview {
            compiler: Compiler::new(state.test.clone()),
            state,
            gui: Widgets {
                render_view,
                main_window,
            },
        }
    }
}

impl Update for Langview {
    type Model = State;
    type ModelParam = (String, String, NotifyReceiver);
    type Msg = Msg;

    fn model(relm: &Relm<Self>, (lang, test, mut recv): Self::ModelParam) -> State {
        let stream = relm.stream().clone();
        let (channel, sender) = Channel::new(move |num| {
            stream.emit(Msg::Recompile);
        });
        let x = sender.clone();
        thread::spawn(move || loop {
            thread::sleep(Duration::from_millis(500));
            if recv.try_next().is_ok() {
                sender.send(1);
            }
        });

        State {
            lang,
            test,
            channel,
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Recompile => {
                let b = self.gui.render_view.get_buffer().unwrap();
                let rendered = b
                    .get_text(&b.get_start_iter(), &b.get_end_iter(), false)
                    .unwrap()
                    .to_string();

                let mut f = File::open(&self.state.lang).unwrap();
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
