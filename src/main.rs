const TASKS: usize = 16usize;

/// Simple executer to print a string
struct PrintExecuter {
    msg: &'static str,
}
impl lwos::Execute for PrintExecuter {
    fn execute(&self, _id: lwos::TaskId) {
        println!("{}", self.msg);
    }
}

fn main() {
    let mut hello_executer = PrintExecuter { msg: "Hello" };
    let mut scheduler_executer = PrintExecuter { msg: "scheduler" };
    let mut world_executer = PrintExecuter { msg: "world!\r\n" };

    let mut hello_task = lwos::Task::new(lwos::TaskState::Running, &mut hello_executer);
    let mut scheduler_task = lwos::Task::new(lwos::TaskState::Running, &mut scheduler_executer);
    let mut world_task = lwos::Task::new(lwos::TaskState::Running, &mut world_executer);

    let mut task_ids: [lwos::TaskId; TASKS] = [lwos::INVALID_ID; TASKS];
    let mut scheduler: lwos::Scheduler<TASKS> = lwos::Scheduler::new();

    task_ids[0] = scheduler.add(&mut hello_task).unwrap();
    task_ids[1] = scheduler.add(&mut scheduler_task).unwrap();
    task_ids[2] = scheduler.add(&mut world_task).unwrap();

    scheduler.process(); // prints "hello scheduler world!
    scheduler.get(task_ids[1]).unwrap().suspend(); // disable "scheduler" print task
    scheduler.process(); // prints "hello world!" only
}
