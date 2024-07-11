#[derive(Debug, PartialEq)]
pub enum Error {
    LexError,
    SyntaxError,
    BadKernelConfig,
    TaskNotFound,
    EventNotFound,
    NoReadyTask,
}