extern crate gtk;

use std::path::PathBuf;

use self::gtk::prelude::*;

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

pub fn connect_signals(builder: &'static gtk::Builder) {
    let btnload: gtk::Button = builder
        .get_object("btnload")
        .expect("couldn't get 'btnload'");
    btnload.connect_activate(move |_| {
        // first check if the path is a legit path
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
    });
}
