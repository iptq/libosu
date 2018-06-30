extern crate failure;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate hitsound_copier;

use std::env;

use failure::Error;
use gdk::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;

// from gtk examples
macro_rules! clone {
    (@param _) => ( _ );
    (@param $x:ident) => ( $x );
    ($($n:ident),+ => move || $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move || $body
        }
    );
    ($($n:ident),+ => move |$($p:tt),+| $body:expr) => (
        {
            $( let $n = $n.clone(); )+
            move |$(clone!(@param $p),)+| $body
        }
    );
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(error) => panic!("error: {:?}", error),
    }
}

fn build_ui(app: &gtk::Application) -> Result<(), Error> {
    let glade_src = include_str!("hscopier.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("couldn't get window");
    window.set_application(app);

    window.show_all();
    Ok(())
}

fn run() -> Result<(), Error> {
    let application = gtk::Application::new("a.b.c", gio::ApplicationFlags::empty())?;
    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&env::args().collect::<Vec<_>>());
    Ok(())
}
