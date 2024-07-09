use crate::kernel::Kernel;

#[derive(Debug, Clone)]
pub struct Api {
    pub name: &'static str,
    function: fn(&mut Kernel),
}

impl Api {
    pub fn execute(&self, kernel: &mut Kernel) {
        (self.function)(kernel)
    }
}

pub static SYS_EXIT: Api = Api {
    name: "sys_exit",
    function: |kernel| {
        kernel.exit_task(kernel.state.current_task).expect("Failed to exit task");
        kernel.schedule();
    },
};

pub static SYS_FORK: Api = Api {
    name: "sys_fork",
    function: |kernel| kernel.new_task(),
};
