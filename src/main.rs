extern crate die;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate version;

mod config;
mod swappy;

use swappy::State;

fn main() {
    swappy::init();
    State::new();
}
