#[derive(Debug)]
pub enum Error {
    LexError,
    SyntaxError,
    BadKernelConfig,
    TaskNotFound,
    EventNotFound,
    NoReadyTask,
}