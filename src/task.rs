
/// Define type used as Task identifier.
/// 
pub type TaskId = usize;

/// Defines the invalid task ID value.
/// 
pub const INVALID_ID: usize = usize::MAX;


#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum TaskState {
    Waiting   = 0,
    Suspended = 1,
    Running   = 2
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Task {
    pub state: TaskState,
    pub id: TaskId,
    pub func: fn()
}

/// Default task handler which does nothing
/// TODO: Thing about Option(f()) and none as default.
/// 
pub fn nop() {}

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

    /// Initializes a task structure.
    pub const fn init(state: TaskState, id : TaskId, func: fn()) -> Self {

        Task { state, id, func }
    }

    /// Suspends a task to no longer schedule it
    /// 
    /// # Examples
    ///
    /// ```
    /// use lwos::task::{Task, TaskState};
    ///
    /// let mut task = Task::new();
    /// task.suspend();
    /// assert_eq!(task.state, TaskState::Suspended);
    /// ```
    pub fn suspend(&mut self) {
        self.state = TaskState::Suspended;
    }

    /// Resumes a task schedule it again
    /// 
    /// # Examples
    ///
    /// ```
    /// use lwos::task::{Task, TaskState};
    ///
    /// let mut task = Task::new();
    /// task.resume();
    /// assert_eq!(task.state, TaskState::Running);
    /// ```
    pub fn resume(&mut self) {
        self.state = TaskState::Running;
    }

    /// Tries to execute the task dependend on status
    /// 
    pub fn execute(&self) {
        match self.state {
            TaskState::Running => {
                (self.func)()
            },
            TaskState::Waiting => {
                {
                    // TODO: Signal processing
                }
            },
            TaskState::Suspended => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_new()  {
        assert_eq!(Task::new(), Task {id: INVALID_ID, func: nop, state: TaskState::Suspended});
    }

    #[test]
    fn task_init()  {
        let t = Task::init(TaskState::Running, 42, task_init);
        assert_eq!(t, Task {id: 42, func: task_init, state: TaskState::Running});
    }
    #[test]
    fn task_suspend_resume() {
        let mut task = Task::new();
        assert_eq!(task.state, TaskState::Suspended);
        task.resume();
        assert_eq!(task.state, TaskState::Running);
        task.suspend();
        assert_eq!(task.state, TaskState::Suspended);
    }
}