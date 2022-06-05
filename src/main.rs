mod config;
mod swappy;

use swappy::State;

fn main() {
    swappy::init();
    State::new();
}
