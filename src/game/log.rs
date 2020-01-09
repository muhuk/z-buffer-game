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

use std::mem::replace;
use std::sync::Mutex;

/// Game logs that show up in the UI (bottom panel).
///
/// Counterintuitively you need a [Write](specs::Write) to **read** from the
/// logs, and a [Read](specs::Read) to **write** to logs.  This means multiple
/// systems can write to the logs simultaneously, but there can be one reader
/// at a time.  Synchronization across the writers are done via a `Mutex`.
#[derive(Clone, Debug)]
pub struct LogEntry(String);

impl LogEntry {
    pub fn new(contents: &str) -> LogEntry {
        LogEntry(contents.to_owned())
    }

    pub fn contents(&self) -> &String {
        &self.0
    }
}

#[derive(Debug, Default)]
pub struct GameLog {
    entries: Mutex<Vec<LogEntry>>,
}

impl GameLog {
    pub fn take(&mut self) -> Vec<LogEntry> {
        let new_msgs = Mutex::new(Vec::new());
        let msgs = replace(&mut self.entries, new_msgs);
        msgs.into_inner().unwrap()
    }

    pub fn push(&self, message: LogEntry) {
        self.entries.lock().unwrap().push(message);
    }
}
