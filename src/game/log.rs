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
