use super::task::Task;
use super::task::TaskState;

pub struct Scheduler<const SIZE : usize> {
    pub tasks: [Task; SIZE],
}


impl<const SIZE : usize> Scheduler<SIZE> {

    pub fn process(&self) {
        for task in self.tasks.iter() {
            task.process();
        }
    }

    pub fn add(&mut self, func: fn(), state: TaskState ) {
        for task in self.tasks.iter_mut() {
            if task.func.is_none() {
                task.init(state, func);
                break;
            }
        }
    }
}
