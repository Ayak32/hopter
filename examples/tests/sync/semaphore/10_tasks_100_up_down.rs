#![no_main]
#![no_std]

extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use cortex_m::asm::delay;
use hopter::{boot::main, debug::semihosting, hprintln, schedule, sync::Semaphore};

static SEMAPHORE: Semaphore = Semaphore::new(10, 5);
static TASK_COMPLETION_COUNTER: AtomicUsize = AtomicUsize::new(0);
const TOTAL_TASKS: usize = 10;

#[main]
fn main(_: cortex_m::Peripherals) {
    // Start 10 tasks
    for i in 0..TOTAL_TASKS {
        schedule::start_task((2 + i) as u8, |_| task(), (), 0, 4).unwrap();
    }
    schedule::change_current_task_priority(10).unwrap();
    check();
}

// Task function that will run independently
fn task() {
    for _ in 0..100 {
        SEMAPHORE.up();
        SEMAPHORE.down();
    }
    // Increment the task completion counter
    TASK_COMPLETION_COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn check() {
    let completed_tasks = TASK_COMPLETION_COUNTER.load(Ordering::SeqCst);

    if completed_tasks == TOTAL_TASKS {
        // All tasks completed, check semaphore count
        let final_count = SEMAPHORE.count();
        // Check if the count matches the initial value
        if final_count == 5 {
            hprintln!("Test Passed");
            semihosting::terminate(true);
        } else {
            hprintln!("Test Failed");
            semihosting::terminate(false);
        }
    }
}
