use rand::Rng;

use crate::state::*;

#[derive(Clone)]
pub struct Scheduler {
    schedule: fn(&AbstractState) -> Option<TaskId>,
}

impl Scheduler {
    pub fn schedule(&self, state: &AbstractState) -> Option<TaskId> {
        (self.schedule)(state)
    }
}

pub static FIFO_SCHEDULER: Scheduler = Scheduler {
    schedule: |state: &AbstractState| -> Option<TaskId> {
        state
            .tasks
            .iter()
            .find(|task| task.status == TaskStatus::Ready)
            .map(|task| task.id)
    },
};

pub static RANDOM_SCHEDULER: Scheduler = Scheduler {
    schedule: |state: &AbstractState| -> Option<TaskId> {
        let ready = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Ready)
            .collect::<Vec<_>>();
        ready
            .iter()
            .nth(rand::thread_rng().gen_range(0..ready.len()))
            .map(|task| task.id)
    },
};
