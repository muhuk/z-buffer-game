use std::mem::replace;
use std::sync::Mutex;

#[derive(Clone, Debug)]
pub struct Message(String);

impl Message {
    pub fn new(contents: String) -> Message {
        Message(contents)
    }
}

#[derive(Debug, Default)]
pub struct Messages {
    messages: Mutex<Vec<Message>>,
}

impl Messages {
    pub fn take(&mut self) -> Vec<Message> {
        let new_msgs = Mutex::new(Vec::new());
        let msgs = replace(&mut self.messages, new_msgs);
        msgs.into_inner().unwrap()
    }

    pub fn push(&self, message: Message) {
        self.messages.lock().unwrap().push(message);
    }
}
