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

/// Example for mutable executer
struct CountExecuter {
    count: usize,
}

impl lwos::Execute for CountExecuter {
    fn execute(&mut self, _id: lwos::TaskId) {
        println!("CountExecuter {}", self.count);
        self.count = self.count + 1;
    }
}

fn main() {
    let mut hello_executer = PrintExecuter { msg: "Hello" };
    let mut scheduler_executer = PrintExecuter { msg: "scheduler" };
    let mut world_executer = PrintExecuter { msg: "world!\r\n" };
    let mut counter_executer = CountExecuter { count: 0usize };

    let mut hello_task = lwos::Task::new(lwos::TaskState::Running, &mut hello_executer);
    let mut scheduler_task = lwos::Task::new(lwos::TaskState::Running, &mut scheduler_executer);
    let mut world_task = lwos::Task::new(lwos::TaskState::Running, &mut world_executer);
    let mut counter_task = lwos::Task::new(lwos::TaskState::Running, &mut counter_executer);

    let mut task_ids: [lwos::TaskId; TASKS] = [lwos::INVALID_ID; TASKS];
    let mut scheduler: lwos::Scheduler<TASKS> = lwos::Scheduler::new();

    task_ids[0] = scheduler.add(&mut hello_task).unwrap();
    task_ids[1] = scheduler.add(&mut scheduler_task).unwrap();
    task_ids[2] = scheduler.add(&mut world_task).unwrap();
    task_ids[3] = scheduler.add(&mut counter_task).unwrap();

    scheduler.process(); // prints "hello scheduler world!
    scheduler.get(task_ids[1]).unwrap().suspend(); // disable "scheduler" print task
    scheduler.process(); // prints "hello world!" only
}
