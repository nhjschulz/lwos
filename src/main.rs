use lwos::scheduler::*;
use lwos::task;
use lwos::task::*;

const TASKS:usize= 16usize;

/// Simple executer to print a string
struct PrintExecuter {
    msg: &'static str
}
impl task::Execute for PrintExecuter {
    fn execute(&mut self, _id : TaskId) {
        println!("{}", self.msg);
    }
}

fn main() {
    let mut scheduler: Scheduler::<TASKS> = Scheduler::new();

    let mut hello_task = PrintExecuter { msg: &"Hello"};
    let mut scheduler_task = PrintExecuter { msg: &"scheduler"};
    let mut world_task = PrintExecuter { msg: &"world!\r\n"};

    let mut task_ids: [TaskId; TASKS]= [INVALID_ID; TASKS];

    task_ids[0] = scheduler.add(&mut hello_task, TaskState::Running).unwrap();
    task_ids[1] = scheduler.add(&mut scheduler_task, TaskState::Running).unwrap();
    task_ids[2] = scheduler.add(&mut world_task, TaskState::Running).unwrap();
 
    scheduler.process();  // prints "hello scheduler world! 

    scheduler.get(task_ids[1]).unwrap().suspend();  // disable "scheduler" print task

    scheduler.process(); // prints "hello world!" only
            
}