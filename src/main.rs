use lwos::scheduler::*;
use lwos::task::*;

const TASKS:usize= 16usize;


static mut SCHEDULER: Scheduler::<TASKS> = Scheduler::<TASKS> { 
    tasks: [Task::new(); TASKS]
};


fn my_task_func1() {
    print!("Hello ");
}

fn my_task_func2() {
    print!("scheduler ");
}

fn my_task_func3() {
    println!("world!\r\n");
}

fn main() {

    unsafe {
        SCHEDULER.add(my_task_func1, TaskState::RUNNING);
        SCHEDULER.add(my_task_func2, TaskState::RUNNING);
        SCHEDULER.add(my_task_func3, TaskState::RUNNING);
 
        SCHEDULER.process();
    }
    
}