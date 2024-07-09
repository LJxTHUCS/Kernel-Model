#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(once_cell_get_mut)]

mod api;
mod kernel;
mod scheduler;
mod state;
mod error;
mod parse;

use api::*;
use kernel::*;
use scheduler::*;
use state::*;

fn main() {
    let sequence = vec!["sys_fork", "sys_exit", "sys_exit"];
    let mut model = Kernel::new(
        vec![SYS_EXIT.clone(), SYS_FORK.clone()],
        FIFO_SCHEDULER.clone(),
        AbstractState {
            current_task: 0,
            tasks: vec![TaskControlBlock {
                id: 0,
                status: TaskStatus::Running,
            }],
        },
    );
    for api in sequence {
        println!("Current state: {:?}", model.state);
        println!("Executing: {}", api);
        model.execute(api);
    }
}
