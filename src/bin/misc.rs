#![allow(unused)]

fn main() {
    let boxed = Box::new(42);
    let option = Some(boxed);

    // Boxed value is now owned by the Option
    // Compiler error:
    // use of moved value: `boxed`
    /*
    let owned = boxed;
    */

    if let Some(owned) = option {
        // Box is moved out of Option
        let result = *owned;
    }

    // Compiler error - use of moved value
    // if let Some(_owned) = option {}









    let boxed = Box::new(42);
    let borrowed = &boxed;
    let option = Some(borrowed);

    if let Some(not_owned) = option {
        // Compiler error - can not move from behind a borrow!
        /*
        let tst = *not_owned;
        */
    }
}
