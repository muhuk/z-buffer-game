use stage::{Stage, StageTransition};
use std::collections::VecDeque;
use std::time::Duration;
use tcod::input::{self as tcod_input, Event as TcodEvent};
use tcod::system::{get_elapsed_time, sleep};
use ui::{self, UI};

const DEFAULT_EVENT_QUEUE_SIZE: usize = 20;

type Event = TcodEvent;

/// Game state.
pub struct Game {
    pub ui: UI,
    pub stage: Stage,
    pub events: VecDeque<Event>,
    pub dt: u32,
    pub time: u64,
}

impl Game {
    pub fn init() -> Game {
        Game {
            ui: ui::initialize(),
            stage: Stage::Menu,
            events: VecDeque::with_capacity(DEFAULT_EVENT_QUEUE_SIZE),
            dt: 0,
            time: 0,
        }
    }

    pub fn main_loop(&mut self) {
        self.ui.root_console.flush();
        while !self.ui.root_console.window_closed() {
            self.update_time();
            self.queue_events();
            match self.stage.tick(self.dt) {
                StageTransition::Continue => (),
                StageTransition::SwitchTo(_) => unimplemented!(),
            }
            ui::draw(self);
        }
    }

    fn push_event(&mut self, e: Event) {
        self.events.push_back(e);
    }

    fn pop_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }

    fn queue_events(&mut self) {
        for (_, e) in tcod_input::events() {
            self.push_event(e);
        }
    }

    fn to_millis(t: &Duration) -> u64 {
        t.as_secs() * 1000 + t.subsec_millis() as u64
    }

    fn update_time(&mut self) {
        let old_time = self.time;
        self.time = Self::to_millis(&get_elapsed_time());
        self.dt = (self.time - old_time) as u32;
    }
}
