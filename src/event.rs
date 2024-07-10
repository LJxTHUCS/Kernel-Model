use crate::parse::{EventAction, EventDef};

#[derive(Debug)]
pub struct Event {
    name: String,
    actions: Vec<EventAction>,
}

impl Event {
    pub fn new(def: EventDef) -> Self {
        Self {
            name: def.name.0,
            actions: def.body,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn actions(&self) -> &[EventAction] {
        &self.actions
    }
}
