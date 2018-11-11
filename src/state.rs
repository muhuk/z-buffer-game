use ui::{self, UI};

/// Application state.
pub struct State {
    pub ui: UI,
}

pub fn initialize() -> State {
    State {
        ui: ui::initialize(),
    }
}
