use crate::{parse::SchedulerType, state::*};
use rand::Rng;

#[derive(Clone)]
pub struct Scheduler {
    type_: SchedulerType,
}

impl Scheduler {
    pub fn new(type_: SchedulerType) -> Self {
        Self { type_ }
    }
    pub fn type_(&self) -> SchedulerType {
        self.type_
    }
    /// Returns the id of the next task to be executed
    pub fn schedule(&self, state: &AbstractState) -> Option<TaskId> {
        match self.type_ {
            SchedulerType::Fifo => Self::fifo(state),
            SchedulerType::Random => Self::random(state),
        }
    }

    // Methods
    fn fifo(state: &AbstractState) -> Option<TaskId> {
        state
            .tasks
            .iter()
            .find(|task| task.status == TaskStatus::Ready)
            .map(|task| task.id)
    }
    fn random(state: &AbstractState) -> Option<TaskId> {
        let ready = state
            .tasks
            .iter()
            .filter(|task| task.status == TaskStatus::Ready)
            .collect::<Vec<_>>();
        if ready.is_empty() {
            return None;
        }
        ready
            .iter()
            .nth(rand::thread_rng().gen_range(0..ready.len()))
            .map(|task| task.id)
    }
}
