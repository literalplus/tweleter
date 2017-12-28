extern crate ansi_term;

mod misc;
mod ui;

use ui::TweleterUi;

fn main() {
    misc::banner::print_banner();
    ui::gtk::GtkUi::run_ui();
}
