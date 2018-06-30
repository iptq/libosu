extern crate failure;
extern crate gio;
extern crate gtk;
extern crate hitsound_copier;

use std::env;

use failure::Error;
use gio::prelude::*;
use gtk::prelude::*;

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

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);
    window.show_all();
}

fn run() -> Result<(), Error> {
    let application = gtk::Application::new("a.b.c", gio::ApplicationFlags::empty())?;
    application.connect_startup(|app| build_ui(app));
    application.connect_activate(|_| {});

    application.run(&env::args().collect::<Vec<_>>());
    Ok(())
}
