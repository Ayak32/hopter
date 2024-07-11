#![no_main]
#![no_std]

extern crate alloc;
use core::sync::atomic::{AtomicUsize, Ordering};
use hopter::{boot::main, debug::semihosting, schedule, hprintln, sync::Mutex};


#[main]
fn main(_: cortex_m::Peripherals) {

    let mutex = Mutex::new(23);
    panic!();
  catch_unwind_with_arg()
  let gaurd = mutex.lock();
  mutex.is_poisoned()



// REFERENCE CODE:
//         // Simulate a panic while holding the mutex
//         let result = panic::catch_unwind(AssertUnwindSafe({
//             let mutex = Arc::clone(&mutex);
//             move || {
//                 let _guard = mutex.lock();
//                 // Simulate a panic
//                 panic!("Simulating panic while holding the mutex");
//             }
//         }));

//         // Check that the panic was caught
//         assert!(result.is_err());

//         // Check that the mutex is poisoned
//         assert!(mutex.is_poisoned(), "Mutex should be poisoned");

}
