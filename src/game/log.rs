use std::mem::replace;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct LogEntry(String);

impl LogEntry {
    pub fn new(contents: String) -> LogEntry {
        LogEntry(contents)
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
