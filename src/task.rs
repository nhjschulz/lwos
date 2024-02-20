// ************************************************************************************************
// DESCRIPTION
// ************************************************************************************************

//! # Task definition
//! - This module provides the task handling functionality.
//! - A task is a function which can be executed by the scheduler.
//! - The task can be in different states like waiting, suspended or running.

// ************************************************************************************************
// MODULES
// ************************************************************************************************

// ************************************************************************************************
// TRAITS
// ************************************************************************************************

pub trait Execute {
    fn execute(&mut self, id: TaskId);
}

// ************************************************************************************************
// TYPES AND STRUCTURES
// ************************************************************************************************

/// Define type used as Task identifier.
///
pub type TaskId = usize;

/// Task structure
pub struct Task<'a> {
    pub state: TaskState,
    pub id: TaskId,
    pub func: &'a mut dyn Execute,
}

#[derive(Debug, PartialEq)]
/// Possible Task States.
pub enum TaskState {
    Waiting = 0,
    Suspended = 1,
    Running = 2,
}

/// Default task handler which does nothing
///
struct NopExecuter {}

// ************************************************************************************************
// CONSTANTS
// ************************************************************************************************

/// Defines the invalid task ID value.
///
pub const INVALID_ID: usize = usize::MAX;

// ************************************************************************************************
// LOCAL VARIABLES
// ************************************************************************************************

// ************************************************************************************************
// IMPLEMENTATIONS
// ************************************************************************************************

impl Execute for NopExecuter {
    fn execute(&mut self, _id: TaskId) {}
}

impl<'a> Task<'a> {
    /// Initializes a task structure with defaults
    ///
    /*
        pub const fn new() -> Self {

            Task {
                state: TaskState::Suspended,
                id: INVALID_ID,
                func: &mut NOP
            }
        }
    */
    /// Initializes a task structure.
    pub fn init(state: TaskState, id: TaskId, func: &'a mut dyn Execute) -> Self {
        Task { state, id, func }
    }

    /// Suspends a task to no longer schedule it
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::task::{Task, TaskState, TaskId, Execute};
    ///
    /// struct SomeExecuter {}
    /// impl Execute for SomeExecuter {
    ///     fn execute(&mut self, _id : TaskId) {
    ///     }
    /// }
    /// let mut executer = SomeExecuter {};
    /// let mut t: Task<'_> = Task::init(TaskState::Running, 42, &mut executer);
    /// t.suspend();
    /// assert_eq!(t.state, TaskState::Suspended);
    /// ```
    pub fn suspend(&mut self) {
        self.state = TaskState::Suspended;
    }

    /// Resume a task to execute it again
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::task::{Task, TaskState, TaskId, Execute};
    ///
    /// struct SomeExecuter {}
    /// impl Execute for SomeExecuter {
    ///     fn execute(&mut self, _id : TaskId) {
    ///     }
    /// }
    /// let mut executer = SomeExecuter {};
    /// let mut t: Task<'_> = Task::init(TaskState::Suspended, 42, &mut executer);
    /// t.resume();
    /// assert_eq!(t.state, TaskState::Running);
    /// ```
    pub fn resume(&mut self) {
        self.state = TaskState::Running;
    }

    /// Tries to execute the task dependend on status
    ///
    pub fn process(&mut self, id: TaskId) {
        match self.state {
            TaskState::Running => {
                self.func.execute(id);
            }
            TaskState::Waiting => {
                {
                    // TODO: Signal processing
                }
            }
            TaskState::Suspended => (),
        }
    }
}

// ************************************************************************************************
// TESTS
// ************************************************************************************************

#[cfg(test)]
mod tests {
    use super::*;

    struct SomeExecuter {}
    impl Execute for SomeExecuter {
        fn execute(&mut self, _id: TaskId) {}
    }

    #[test]
    fn task_init() {
        let mut task_executer: SomeExecuter = SomeExecuter {};
        let t = Task::init(TaskState::Running, 42, &mut task_executer);
        assert_eq!(t.id, 42);
        assert_eq!(t.state, TaskState::Running);
        //assert_eq!(t.func., &mut taskExecuter);
    }
    #[test]
    fn task_suspend_resume() {
        let mut task_executer: SomeExecuter = SomeExecuter {};
        let mut t: Task<'_> = Task::init(TaskState::Suspended, 42, &mut task_executer);

        assert_eq!(t.state, TaskState::Suspended);
        t.resume();
        assert_eq!(t.state, TaskState::Running);
        t.suspend();
        assert_eq!(t.state, TaskState::Suspended);
    }
}
