use crate::data::Direction;

#[derive(Clone, Copy, Debug)]
pub enum GameEvent {
    Move(Direction),
}
