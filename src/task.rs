

pub type TaskId = usize;
pub const INVALID_ID: usize = usize::MAX;

#[derive(Clone, Copy)]
pub enum TaskState {
    Blocked   = 0,
    Suspended = 1,
    Running   = 2
}

#[derive(Clone, Copy)]
pub struct Task {
    pub state: TaskState,
    pub id: TaskId,
    pub func: fn()
}

/// Default task handler which does nothing
/// TODO: Thing about Option(f()) and none as default.
/// 
fn nop() {

}

impl Task {

    /// Initializes a task structure with defaults
    /// 
    pub const fn new() -> Self {

        Task {
            state: TaskState::Suspended,
            id: INVALID_ID,
            func: nop
        }
    }

    /// Initializes s task structure.
    pub fn init(&mut self, state: TaskState, id : TaskId, func: fn()) {

        *self = Task { state, id, func }
    }
}
