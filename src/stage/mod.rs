use std::time::Duration;
use ui::UI;

enum StageTransition {
    Continue,
    SwitchTo { next: Box<Stage> },
}

trait Stage {
    fn tick(&self, dt: Duration, ui: UI) -> StageTransition;
}
