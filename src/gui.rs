use std::time::Duration;
use std::{fs, thread};

use gdk::keys::constants as Keys;
use gdk::keys::Key;
use gdk::{ModifierType as KeyMod, ModifierType};
use gtk::prelude::*;
use gtk::Window;
use relm::{connect, Channel, Relm, Update, Widget};
use relm_derive::Msg;
use sourceview::{BufferExt, View as SourceView};

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
    KeyPress(Key, ModifierType),
    Recompile,
    Quit,
}

impl Widget for Langview {
    type Root = Window;

    fn init_view(&mut self) {
        let lang_src = fs::read_to_string(&self.state.lang).unwrap();
        let test_txt = fs::read_to_string(&self.state.test).unwrap();

        let buffer = self.compiler.compile_buffer(&lang_src);
        buffer.begin_not_undoable_action();
        buffer.set_text(&test_txt);
        buffer.end_not_undoable_action();
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
            connect_key_press_event(_, key),
            return (
                Some(Msg::KeyPress(key.get_keyval(), key.get_state())),
                Inhibit(false)
            )
        );

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
            Msg::KeyPress(key, key_mod) => match (key, key_mod) {
                (Keys::w, KeyMod::CONTROL_MASK) => gtk::main_quit(),
                _ => {
                    println!("{:?}", key_mod)
                }
            },
            Msg::Recompile => {
                let live = self.gui.render_view.get_buffer().unwrap();
                let rendered = live
                    .get_text(&live.get_start_iter(), &live.get_end_iter(), false)
                    .unwrap()
                    .to_string();

                let lang_src = fs::read_to_string(&self.state.lang).unwrap();
                let lang_buff = self.compiler.compile_buffer(&lang_src);
                lang_buff.begin_not_undoable_action();
                lang_buff.set_text(&rendered);
                lang_buff.end_not_undoable_action();
                self.gui.render_view.set_buffer(Some(&lang_buff))
            }
            Msg::Quit => gtk::main_quit(),
        }
    }
}
