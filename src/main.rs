#[macro_use] extern crate conrod;
extern crate ansi_term;

mod misc;
mod ui;

fn main() {
    misc::banner::print_banner();
    ui::gtk::GtkUi::run_ui();
}
