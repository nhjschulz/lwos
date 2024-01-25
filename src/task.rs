
#[derive(Clone, Copy)]
pub enum TaskState {
    SUSPENDED = 0,
    RUNNING
}

#[derive(Clone, Copy)]
pub struct Task {
    pub state: TaskState,
    pub func: Option<fn()>
}

impl Task {

    pub const fn new() -> Self {

        Task {
            state: TaskState::SUSPENDED,
            func: None
        }
    }

    pub fn init(&mut self, state: TaskState, func: fn()) {

        *self = Task {state: state, func: Some(func) }
    }

    pub fn process(&self) {
        if let TaskState::RUNNING = self.state {
            match self.func {
                Some(func) => { func() },
                None => (),
            }
        }
    }
}
