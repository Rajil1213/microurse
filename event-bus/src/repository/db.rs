use std::sync::{Arc, RwLock};

use crate::models::Event;

#[derive(Debug, Clone)]
pub struct Db {
    events: Arc<RwLock<Vec<Event>>>,
}

impl Default for Db {
    fn default() -> Self {
        Self {
            events: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Db {
    pub fn fetch(&self) -> Vec<Event> {
        let events = self.events.read().unwrap();

        events.clone()
    }

    pub fn add(&self, event: &Event) {
        let mut events = self.events.write().unwrap();

        events.push(event.clone());
    }
}
