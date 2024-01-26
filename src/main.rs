use lwos::scheduler::*;
use lwos::task::*;

const TASKS:usize= 16usize;


static mut SCHEDULER: Scheduler::<TASKS> = Scheduler::new();


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

    let mut task_ids: [TaskId; TASKS]= [INVALID_ID; TASKS];

    // unsafe needed because of static scheduler. Not needed for stack based.
    unsafe {
        task_ids[0] = SCHEDULER.add(my_task_func1, TaskState::Running).unwrap();
        task_ids[1] = SCHEDULER.add(my_task_func2, TaskState::Running).unwrap();
        task_ids[2] = SCHEDULER.add(my_task_func3, TaskState::Running).unwrap();
 
        SCHEDULER.process();
    }
    
}