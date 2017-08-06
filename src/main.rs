extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, MessageDialog, DialogFlags, MessageType, ButtonsType};

fn main() {
    if gtk::init().is_err() {
        println!("Well, that didn't last long.");
        panic!("a minute of silence to our lost brother, gtk");
    }

    MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       "Hello World").run();
}
