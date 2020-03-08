// © Copyright 2019-2020, Atamert Ölçgen
//
// This file is part of z-buffer-game.
//
// z-buffer-game is free software: you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at your
// option) any later version.
//
// z-buffer-game is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public
// License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with z-buffer-game.  If not, see <https://www.gnu.org/licenses/>.

#[derive(Clone, Copy, Debug, Default)]
pub struct Time {
    dt_millis: u32,
    t_millis: u64,
    game_time_days: u16,
    game_time_hours: u8,
    game_time_minutes: u8,
    game_time_millis: u32,
}

impl Time {
    pub fn advance_dt(&mut self, dt: u32) {
        self.dt_millis = dt;
        self.t_millis += u64::from(dt);
    }

    pub fn dt_millis(self) -> u32 {
        self.dt_millis
    }

    pub fn game_time_days(self) -> u16 {
        self.game_time_days
    }

    pub fn game_time_hours(self) -> u8 {
        self.game_time_hours
    }

    pub fn game_time_millis(self) -> u32 {
        self.game_time_millis
    }

    pub fn game_time_minutes(self) -> u8 {
        self.game_time_minutes
    }

    pub fn set_game_time(
        &mut self,
        days: u16,
        hours: u8,
        minutes: u8,
        millis: u32,
    ) {
        self.game_time_days = days;
        self.game_time_hours = hours;
        self.game_time_minutes = minutes;
        self.game_time_millis = millis;
    }

    pub fn t_millis(self) -> u64 {
        self.t_millis
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn advance_dt_increments_both_dt_and_t() {
        let mut time = Time::default();
        time.advance_dt(1000);
        assert_eq!(1000, time.dt_millis());
        assert_eq!(1000, time.t_millis());
        time.advance_dt(2000);
        assert_eq!(2000, time.dt_millis());
        assert_eq!(3000, time.t_millis());
        time.advance_dt(250);
        assert_eq!(250, time.dt_millis());
        assert_eq!(3250, time.t_millis());
    }
}
