

const TASKS: usize = 16usize;

/// Simple executer to print a string
struct PrintExecuter {
    msg: &'static str,
}
impl lwos::Execute for PrintExecuter {
    fn execute(&mut self, _id: lwos::TaskId) {
        println!("{}", self.msg);
    }
}

fn main() {
    let mut scheduler: lwos::Scheduler<TASKS> = lwos::Scheduler::new();

    let mut hello_task = PrintExecuter { msg: "Hello" };
    let mut scheduler_task = PrintExecuter { msg: "scheduler" };
    let mut world_task = PrintExecuter { msg: "world!\r\n" };

    let mut task_ids: [lwos::TaskId; TASKS] = [lwos::INVALID_ID; TASKS];

    task_ids[0] = scheduler
        .add(&mut hello_task, lwos::TaskState::Running)
        .unwrap();
    task_ids[1] = scheduler
        .add(&mut scheduler_task, lwos::TaskState::Running)
        .unwrap();
    task_ids[2] = scheduler
        .add(&mut world_task, lwos::TaskState::Running)
        .unwrap();

    scheduler.process(); // prints "hello scheduler world!

    scheduler.get(task_ids[1]).unwrap().suspend(); // disable "scheduler" print task

    scheduler.process(); // prints "hello world!" only
}
