//! The scheduler works in cooperative mode. You can add or remove tasks. The
//! tasks will be executed in the order they are registered.

use super::task::INVALID_ID;

use super::task::Task;
use super::task::TaskId;
use super::task::TaskState;

/// Definition for the Scheduler data structure which can 
/// manage a set of task stored internally as an array.
pub struct Scheduler<const SIZE : usize> {
    pub tasks: [Task; SIZE],
}

/// Posible error values from this module.
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Error {
    LimitExceeded,
    NoSuchTaskId,
    InvalidParameter
}

impl<const SIZE : usize> Scheduler<SIZE> {

    /// Creates a scheduler instance with a maximum number of tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwos::scheduler::Scheduler;
    ///
    /// let scheduler: Scheduler::<10> = Scheduler::new();  // allow 10 tasks
    /// ```
    pub const fn new() -> Self {
        Scheduler::<SIZE> { tasks: [Task::new(); SIZE] }
    }

    /// Runs a scheduler process cycle by executing all
    /// active tasks in a simple round robin method.
    /// 
    /// # Examples
    ///
    /// ```
    /// use lwos::scheduler::Scheduler;
    ///
    /// let scheduler: Scheduler::<3> = Scheduler::new();
    /// scheduler.process();
    /// ```
    /// 
    pub fn process(&self) {
        for task in self.tasks.iter() {
            match task.state {
                TaskState::Running => {
                    (task.func)()
                },
                _ => ()
            }
        }
    }

    /// Adds a new task to the scheduler.
    /// 
    /// # Examples
    ///
    /// ```
    /// use lwos::task::{Task, TaskId, TaskState};
    /// use lwos::scheduler::Scheduler;
    /// 
    /// fn task_entry() {}
    /// 
    /// let mut scheduler: Scheduler::<3> = Scheduler::new();
    /// let task_id = scheduler.add(task_entry,TaskState::Running).unwrap();
    /// ```
    /// 
    pub fn add(&mut self, func: fn(), state: TaskState ) -> Result<TaskId, Error> {

        for (index, task) in self.tasks.iter_mut().enumerate() {
            if let INVALID_ID = task.id {
                * task = Task::init(state, index, func);
                return Ok(index);
            }
        }

        Err(Error::LimitExceeded)
    }

    /// Removes given task from scheduler.
    ///  
    pub fn remove(&mut self, id: TaskId) -> Result<TaskId, Error> {
        match self.get(id) {
            Ok(task) => {
                *task = Task::new();

                Ok(id)
            },
            Err(e) => Err(e)
        }
    }

    pub fn get(&mut self, id: TaskId) -> Result<&mut Task, Error> {
        if SIZE > id {
            if INVALID_ID != self.tasks[id].id {
                
                Ok(&mut self.tasks[id])
            } else {
                Err(Error::NoSuchTaskId)
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

#[cfg(test)]
mod tests {
    use crate::task;

    use super::*;


    fn task_0() {}
    fn task_1() {}
    fn task_2() {}

    #[test]
    fn scheduler_add() {
        let mut scheduler: Scheduler::<1>  = Scheduler::new();

        assert_eq!(scheduler.tasks[0].id, INVALID_ID);
        assert_eq!(scheduler.tasks[0].func as usize, task::nop as usize);
        assert_eq!(scheduler.tasks[0].state, TaskState::Suspended);


        assert_eq!(scheduler.add(task_0, TaskState::Running).unwrap(), 0);

        assert_eq!(scheduler.tasks[0].id, 0);
        assert_eq!(scheduler.tasks[0].func as usize, task_0 as usize);
        assert_eq!(scheduler.tasks[0].state, TaskState::Running);
       
    }

    #[test]
    fn scheduler_add_capacity() {
        let mut scheduler: Scheduler::<3>  = Scheduler::new();

        assert_eq!(scheduler.add(task_0, TaskState::Running).unwrap(), 0);
        assert_eq!(scheduler.add(task_1, TaskState::Running).unwrap(), 1);
        assert_eq!(scheduler.add(task_2, TaskState::Running).unwrap(), 2);
        assert_eq!(scheduler.add(task_2, TaskState::Running).unwrap_err(), Error::LimitExceeded);
    }
    #[test]
    #[should_panic]
    fn scheduler_add_too_much() {
        let mut scheduler: Scheduler::<1>  = Scheduler::new();


        assert_eq!(scheduler.add(task_0, TaskState::Running).unwrap(), 0);
        assert_eq!(scheduler.add(task_1, TaskState::Running).unwrap(), 1);  // <- panics (capacity)
    }

    #[test]
    fn scheduler_remove() {
        let mut scheduler: Scheduler::<1>  = Scheduler::new();

        assert_eq!(scheduler.add(task_0, TaskState::Running).unwrap(), 0);
        assert_eq!(scheduler.remove(0).unwrap(), 0);
        assert_eq!(scheduler.remove(0).unwrap_err(), Error::NoSuchTaskId);
        assert_eq!(scheduler.remove(1).unwrap_err(), Error::InvalidParameter);


    }
}