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
    fn execute(&self, id: TaskId);
}

// ************************************************************************************************
// TYPES AND STRUCTURES
// ************************************************************************************************

///  Type used as Task identifier.
///
pub type TaskId = usize;

/// Task structure
pub struct Task<'a> {
    pub state: TaskState,
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
    fn execute(&self, _id: TaskId) {}
}

impl<'a> Task<'a> {
    /// Initializes a task structure.
    ///
    pub fn new(state: TaskState, func: &'a mut dyn Execute) -> Self {
        Task { state, func }
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
    pub fn process(&self, id: TaskId) {
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
        fn execute(&self, _id: TaskId) {}
    }

    #[test]
    fn task_init() {
        let mut task_executer: SomeExecuter = SomeExecuter {};
        let t = Task::new(TaskState::Running, &mut task_executer);
        assert_eq!(t.state, TaskState::Running);
    }

    #[test]
    fn task_suspend_resume() {
        let mut task_executer: SomeExecuter = SomeExecuter {};
        let mut t: Task<'_> = Task::new(TaskState::Suspended, &mut task_executer);

        assert_eq!(t.state, TaskState::Suspended);
        t.resume();
        assert_eq!(t.state, TaskState::Running);
        t.suspend();
        assert_eq!(t.state, TaskState::Suspended);
    }
}
