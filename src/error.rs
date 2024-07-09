#[derive(Debug)]
pub enum Error {
    TaskNotFound,
    ApiNotFound,
    NoReadyTask,
}