#[derive(Debug)]
pub enum Error {
    Lex,
    Syntax,
    TaskNotFound,
    ApiNotFound,
    NoReadyTask,
}