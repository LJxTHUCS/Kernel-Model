pub type TaskId = u32;

#[derive(Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Ready,
    Running,
}

#[derive(Debug)]
pub struct TaskControlBlock {
    pub id: TaskId,
    pub status: TaskStatus,
}

#[derive(Debug)]
pub struct AbstractState {
    pub current_task: TaskId,
    pub tasks: Vec<TaskControlBlock>,
}