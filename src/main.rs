mod config;
mod swappy;

use swappy::State;

fn main() {
    let mut state = State::new();
    swappy::init(state);
}
