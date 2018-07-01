extern crate failure;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate hitsound_copier;

use std::env;
use std::path::PathBuf;

use failure::Error;
use gdk::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Builder, BuilderExt};
use hitsound_copier::gui;

// from the gtk examples
// make moving clones into closures more convenient
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
    let builder = Builder::new_from_string(include_str!("hscopier.glade"));
    let window: gtk::ApplicationWindow = builder.get_object("window").expect("couldn't get window");
    window.set_application(app);

    let btnchoose: gtk::Button = builder
        .get_object("btnchoose")
        .expect("couldn't get btnchoose");
    let btnload: gtk::Button = builder.get_object("btnload").expect("couldn't get btnload");
    let inputfolder: gtk::Entry = builder
        .get_object("inputfolder")
        .expect("couldn't gegt inputfolder");

    btnchoose.connect_clicked(clone!(window => move |_| {
        let folderpicker = gtk::FileChooserDialog::new(
            Some("Choose Folder"),
            Some(&window),
            gtk::FileChooserAction::SelectFolder,
        ); 
        folderpicker.add_buttons(&[
            ("Cancel", gtk::ResponseType::Cancel.into()),
            ("Open", gtk::ResponseType::Ok.into()),
        ]);
        let response = folderpicker.run();
        println!("response: {:?}", response);
        if response == gtk::ResponseType::Ok.into() {
            let path = folderpicker
                .get_filename()
                .expect("couldn't get filename")
                .into_os_string()
                .into_string()
                .unwrap();
            inputfolder.set_text(&path);
        }
        folderpicker.destroy();
    }));

    /*    let btnload_action = Box::new(clone!(builder => move |_| {
        let input: gtk::Entry = builder
            .get_object("inputfolder")
            .expect("couldn't get 'inputfolder'");
        let path_str;
        match input.get_text() {
            None => return,
            Some(s) => path_str = s,
        }
        println!("str: {:?}", path_str);
        let path = PathBuf::from(path_str);

        println!("path: {:?}", path);
    }));*/
    // btnload.connect_activate(*btnload_action);
    //btnload.connect_clicked(*btnload_action);

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
