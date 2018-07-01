extern crate failure;
extern crate gdk;
extern crate gio;
extern crate gtk;
extern crate hitsound_copier;
extern crate libosu;

use std::fs::{self,File};
use std::io::Read;
use std::env;
use std::path::PathBuf;

use failure::Error;
use gdk::prelude::*;
use gio::prelude::*;
use gtk::prelude::*;
use libosu::*;
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

   btnload.connect_clicked(clone!(builder => move |_| {
        let input: gtk::Entry = builder
            .get_object("inputfolder")
            .expect("couldn't get 'inputfolder'");
        let path_str;
        match input.get_text() {
            None => return,
            Some(s) => path_str = s,
        }
        println!("str: {:?}", path_str);
        let path = PathBuf::from(&path_str);

        println!("path: {:?}", path);
        let mut names = Vec::new();
        for name in fs::read_dir(path_str).unwrap() {
            if let Ok(de) = name {
                let p = de.path();
                let p1 = p.clone();
                let name = p.file_name().unwrap().to_str().unwrap();
                if !name.ends_with(".osu") {
                    continue;
                }
                let mut f = File::open(p1).expect("couldn't open file");
                let mut contents = String::new();
                f.read_to_string(&mut contents).expect("couldn't read");
                let map = Beatmap::deserialize(contents).expect("couldn't parse");
                println!("name: {:?}", name);
                let row = gtk::ListBoxRow::new();
                let label = gtk::Label::new(Some(map.difficulty_name.as_ref()));
                label.set_justify(gtk::Justification::Left);
                row.add(&label);
                names.push(row);
            } else {
                println!("wtf");
            }
        }
        println!("names: {:?}", names);
        
        let src_box: gtk::ListBox = builder.get_object("src_files").expect("couldn't get src_files");
        let dst_box: gtk::ListBox = builder.get_object("dst_files").expect("couldn't get dst_files");
        for row in names {
            src_box.add(&row.clone());
            dst_box.add(&row.clone());
        }
        src_box.show_all();
        dst_box.show_all();
        builder.get_object::<gtk::Frame>("area").expect("couldn't get area").set_sensitive(true);
    }));

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
