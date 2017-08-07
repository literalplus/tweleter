extern crate qt_core;
extern crate ansi_term;

use qt_core::core_application::CoreApplication;

mod misc;

fn main() {
    CoreApplication::create_and_exit(|app| {
        misc::banner::print_banner();
        CoreApplication::exec()
    })
}
