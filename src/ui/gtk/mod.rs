extern crate gtk;
use self::gtk::{ButtonExt, DialogExt, WidgetExt};

pub struct GtkUi {}

macro_rules! gtkclone {
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
            move |$(gtkclone!(@param $p),)+| $body
        }
    );
}

impl GtkUi {
    pub fn run_ui() {
        if gtk::init().is_err() {
            println!("Well, that didn't last long.");
            panic!("a minute of silence to our lost brother, gtk");
        }

        let glade_src = include_str!("main_window.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let window: gtk::Window = builder.get_object("tw_win").unwrap();
        let button: gtk::Button = builder.get_object("tw_btn_test").unwrap();
        let dialog: gtk::MessageDialog = builder.get_object("td_win").unwrap();

        button.connect_clicked(move |_| {
            dialog.run();
            dialog.hide();
        });

        window.show_all();

        gtk::main();
    }
}