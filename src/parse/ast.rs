use super::lexer::TokenKind;

// Program
#[derive(Debug)]
pub struct Model {
    pub event_defs: Vec<EventDef>,
    pub kernel_def: KernelDef,
}

#[derive(Debug)]
pub struct EventDef {
    pub name: Identifier,
    pub body: Vec<EventAction>,
}

#[derive(Debug)]
pub enum EventAction {
    Shutdown,
    NewTask,
    Exit,
    Sched,
    Stop,
}

#[derive(Debug)]
pub struct KernelDef {
    pub configs: Vec<KernelConfig>,
}

#[derive(Debug)]
pub enum KernelConfig {
    Events(Vec<Identifier>),
    Scheduler(SchedulerType),
}

#[derive(Debug)]
pub enum SchedulerType {
    Fifo,
}

impl From<TokenKind> for EventAction {
    fn from(token: TokenKind) -> Self {
        match token {
            TokenKind::Shutdown => EventAction::Shutdown,
            TokenKind::NewTask => EventAction::NewTask,
            TokenKind::Exit => EventAction::Exit,
            TokenKind::Sched => EventAction::Sched,
            TokenKind::Stop => EventAction::Stop,
            _ => panic!("Invalid event action"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Identifier(pub String);

impl From<Identifier> for String {
    fn from(id: Identifier) -> Self {
        id.0
    }
}

impl AsRef<String> for Identifier {
    fn as_ref(&self) -> &String {
        &self.0
    }
}

impl AsMut<String> for Identifier {
    fn as_mut(&mut self) -> &mut String {
        &mut self.0
    }
}
