use crate::api::*;
use crate::error::Error;
use crate::scheduler::*;
use crate::state::*;

pub struct Kernel {
    pub apis: Vec<Api>,
    pub scheduler: Scheduler,
    pub state: AbstractState,
    pub next_task_id: TaskId,
}

impl Kernel {
    pub fn new(apis: Vec<Api>, scheduler: Scheduler, state: AbstractState) -> Self {
        Self {
            apis,
            scheduler,
            state,
            next_task_id: 1,
        }
    }
    pub fn state(&self) -> &AbstractState {
        &self.state
    }
    pub fn execute(&mut self, api: &str) -> Result<(), Error> {
        let api = self
            .apis
            .iter()
            .find(|a| a.name == api)
            .ok_or(Error::ApiNotFound)?
            .clone();
        api.execute(self);
        Ok(())
    }
    pub fn new_task(&mut self) {
        self.state.tasks.push({
            TaskControlBlock {
                id: self.next_task_id,
                status: TaskStatus::Ready,
            }
        });
        self.next_task_id += 1;
    }
    pub fn exit_task(&mut self, id: TaskId) -> Result<(), Error> {
        let index = self
            .state
            .tasks
            .iter()
            .position(|task| task.id == id)
            .ok_or(Error::TaskNotFound)?;
        self.state.tasks.remove(index);
        Ok(())
    }
    pub fn schedule(&mut self) -> Result<(), Error> {
        self.scheduler
            .schedule(&self.state)
            .map_or(Err(Error::NoReadyTask), |id| {
                self.state
                    .tasks
                    .iter_mut()
                    .find(|task| task.id == id)
                    .unwrap()
                    .status = TaskStatus::Running;
                self.state.current_task = id;
                Ok(())
            })
    }
}
