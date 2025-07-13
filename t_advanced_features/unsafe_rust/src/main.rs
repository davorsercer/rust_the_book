static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time.
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    unsafe {
        // SAFETY: This is only called from a single thread in `main`.
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }
}

// fn main() {
//     let mut num = 5;

//     let r1 = &raw const num;
//     let r2 = &raw mut num;

//     unsafe {
//         println!("r1 is: {}", *r1);
//         println!("r2 is: {}", *r2);
//     }
// }
