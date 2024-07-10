use crate::error::Error;
use crate::event::*;
use crate::parse::EventAction;
use crate::scheduler::*;
use crate::state::*;

pub struct Kernel {
    events: Vec<Event>,
    scheduler: Scheduler,
    state: AbstractState,
    next_task_id: TaskId,
    shutdown_code: Option<i32>,
}

impl Kernel {
    pub fn new(events: Vec<Event>, scheduler: Scheduler) -> Self {
        let mut kernel = Self {
            events,
            scheduler,
            state: AbstractState::new(),
            next_task_id: 0,
            shutdown_code: None,
        };
        kernel.new_task().unwrap();
        kernel.sched().unwrap();
        kernel
    }
    pub fn state(&self) -> &AbstractState {
        &self.state
    }
    /// Execute a given event
    pub fn execute(&mut self, event: &str) -> Result<(), Error> {
        let event = self
            .events
            .iter()
            .find(|e| e.name() == event)
            .ok_or(Error::EventNotFound)?;
        let actions = event.actions().to_vec();
        for action in actions {
            match action {
                EventAction::Shutdown => self.shutdown(0)?,
                EventAction::NewTask => self.new_task()?,
                EventAction::Exit => self.exit()?,
                EventAction::Sched => self.sched()?,
                EventAction::Stop => self.stop()?,
            }
        }
        Ok(())
    }
    /// Print configurations
    pub fn print_config(&self) {
        println!("[Kernel Model]");
        println!("Scheduler = {:?}", self.scheduler.type_());
        println!(
            "Events = [{}]",
            self.events
                .iter()
                .map(|e| e.name())
                .collect::<Vec<_>>()
                .join(", ")
        );
        println!("");
    }
    /// Print state
    pub fn print_state(&self) {
        println!("[Kernel State]");
        println!("Current Task = {:?}", self.state.current_task);
        println!("Tasks = {:?}", self.state.tasks);
        println!("");
    }

    pub fn shutdown_code(&self) -> Option<i32> {
        self.shutdown_code
    }

    // Built-in operations

    /// Turn off the kernel model
    fn shutdown(&mut self, code: i32) -> Result<(), Error> {
        self.shutdown_code = Some(code);
        Ok(())
    }
    /// Switch to the task given by scheduler
    fn sched(&mut self) -> Result<(), Error> {
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
    /// Stop current task
    fn stop(&mut self) -> Result<(), Error> {
        let cur = self
            .state
            .tasks
            .iter_mut()
            .find(|task| task.id == self.state.current_task)
            .ok_or(Error::TaskNotFound)?;
        cur.status = TaskStatus::Ready;
        Ok(())
    }
    /// Exit current task
    fn exit(&mut self) -> Result<(), Error> {
        let index = self
            .state
            .tasks
            .iter()
            .position(|task| task.id == self.state.current_task)
            .ok_or(Error::TaskNotFound)?;
        self.state.tasks.remove(index);
        Ok(())
    }
    /// Add a new task
    fn new_task(&mut self) -> Result<(), Error> {
        self.state.tasks.push({
            TaskControlBlock {
                id: self.next_task_id,
                status: TaskStatus::Ready,
            }
        });
        self.next_task_id += 1;
        Ok(())
    }
}
