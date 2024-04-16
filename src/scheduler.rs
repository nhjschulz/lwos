//! The scheduler works in cooperative mode. You can add or remove tasks. The
//! tasks will be executed in the order they are registered.

use super::task::*;

/// Definition for the Scheduler data structure which can
/// manage a set of task stored internally as an array.
pub struct Scheduler<'a, const SIZE: usize> {
    tasks: [Option<&'a mut Task<'a>>; SIZE],
}

/// Posible error values from this module.
#[derive(Debug, PartialEq)]
pub enum Error {
    LimitExceeded,
    NoSuchTaskId,
    InvalidParameter,
}

impl<'a, const SIZE: usize> Scheduler<'a, SIZE> {
    const TASK_INIT_NONE: Option<&'a mut Task<'a>> = None;

    /// Creates a scheduler instance with a maximum number of tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::scheduler::Scheduler;
    ///
    /// let scheduler: Scheduler::<10> = Scheduler::new();  // allow 10 tasks
    /// ```
    pub fn new() -> Self {
        Scheduler::<SIZE> {
            tasks: [Self::TASK_INIT_NONE; SIZE],
        }
    }

    /// Runs a scheduler process cycle by executing all
    /// active tasks in a simple round robin method.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::scheduler::Scheduler;
    ///
    /// let mut scheduler: Scheduler::<3> = Scheduler::new();
    /// scheduler.process();
    /// ```
    ///
    pub fn process(&mut self) {
        for (index, item) in self.tasks.iter_mut().enumerate() {
            match item {
                Some(task) => task.process(index),
                None => (),
            }
        }
    }

    /// Adds a new task to the scheduler.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::task::{Task, TaskId, TaskState, Execute};
    /// use lwos::scheduler::Scheduler;
    ///
    /// struct SomeExecuter {}
    /// impl Execute for SomeExecuter {
    ///     fn execute(&mut self, _id : TaskId) {
    ///     }
    /// }
    ///
    /// let mut scheduler: Scheduler::<3> = lwos::Scheduler::new();
    /// let mut executer = SomeExecuter {};
    /// let mut t = lwos::Task::new(lwos::TaskState::Running, &mut executer);
    /// let task_id = scheduler.add(&mut t).unwrap();
    /// ```
    ///   
    pub fn add(&mut self, task: &'a mut Task<'a>) -> Result<TaskId, Error> {
        match self.tasks.iter().position(|x| x.is_none()) {
            Some(id) => {
                self.tasks[id] = Some(task);
                Ok(id)
            }
            None => Err(Error::LimitExceeded),
        }
    }

    /// Removes given task from scheduler.
    ///  
    pub fn remove(&mut self, id: TaskId) -> Result<(), Error> {
        match self.get(id) {
            Ok(_) => {
                self.tasks[id] = Self::TASK_INIT_NONE;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    pub fn get(&mut self, id: TaskId) -> Result<&mut Task<'a>, Error> {
        if SIZE > id {
            match &mut self.tasks[id] {
                Some(task) => Ok(task),
                None => Err(Error::NoSuchTaskId),
            }
        } else {
            Err(Error::InvalidParameter)
        }
    }

    /// Gets the maximum number of tasks supported by this scheduler.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::scheduler::Scheduler;
    ///
    /// let scheduler: Scheduler::<3> = Scheduler::new();
    /// assert_eq!(scheduler.capacity(), 3);
    /// ```
    pub const fn capacity(&self) -> usize {
        SIZE
    }
}

impl<'a, const SIZE: usize> Default for Scheduler<'a, SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct SomeExecuter {}
    impl Execute for SomeExecuter {
        fn execute(&mut self, _id: TaskId) {}
    }

    #[test]
    fn scheduler_add() {
        let mut scheduler: Scheduler<1> = Scheduler::new();
        assert!(scheduler.tasks[0].is_none());

        let mut e1: SomeExecuter = SomeExecuter {};
        let mut t1 = Task::new(TaskState::Running, &mut e1);

        assert_eq!(scheduler.add(&mut t1).unwrap(), 0);

        assert_eq!(
            scheduler.tasks[0].as_ref().unwrap().state,
            TaskState::Running
        );
    }

    #[test]
    fn scheduler_add_capacity() {
        let mut scheduler: Scheduler<3> = Scheduler::new();

        let mut e1: SomeExecuter = SomeExecuter {};
        let mut e2: SomeExecuter = SomeExecuter {};
        let mut e3: SomeExecuter = SomeExecuter {};
        let mut e4: SomeExecuter = SomeExecuter {};

        let mut t1 = Task::new(TaskState::Running, &mut e1);
        let mut t2 = Task::new(TaskState::Running, &mut e2);
        let mut t3 = Task::new(TaskState::Running, &mut e3);
        let mut t4 = Task::new(TaskState::Running, &mut e4);

        assert_eq!(scheduler.add(&mut t1).unwrap(), 0);
        assert_eq!(scheduler.add(&mut t2).unwrap(), 1);
        assert_eq!(scheduler.add(&mut t3).unwrap(), 2);
        assert_eq!(scheduler.add(&mut t4).unwrap_err(), Error::LimitExceeded);
    }

    #[test]
    #[should_panic]
    fn scheduler_add_too_much() {
        let mut scheduler: Scheduler<1> = Scheduler::new();
        let mut e1: SomeExecuter = SomeExecuter {};
        let mut e2: SomeExecuter = SomeExecuter {};
        let mut t1 = Task::new(TaskState::Running, &mut e1);
        let mut t2 = Task::new(TaskState::Running, &mut e2);

        assert_eq!(scheduler.add(&mut t1).unwrap(), 0);
        assert_eq!(scheduler.add(&mut t2).unwrap(), 1); // <- panics (capacity)
    }

    #[test]
    fn scheduler_remove() {
        let mut scheduler: Scheduler<1> = Scheduler::new();
        let mut e1: SomeExecuter = SomeExecuter {};
        let mut t1 = Task::new(TaskState::Running, &mut e1);

        assert_eq!(scheduler.add(&mut t1).unwrap(), 0);
        assert_eq!(scheduler.remove(0), Ok(()));
        assert_eq!(scheduler.remove(0).unwrap_err(), Error::NoSuchTaskId);
        assert_eq!(scheduler.remove(1).unwrap_err(), Error::InvalidParameter);
    }
}
