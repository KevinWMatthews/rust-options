// https://github.com/rust-lang/rfcs/blob/master/text/2005-match-ergonomics.md
// https://blog.rust-lang.org/2017/03/02/lang-ergonomics.html

use std::boxed::Box;

fn main() {
    match_and_move();
    match_and_borrow();
    match_ref_and_move1();
    match_ref_and_move2();
    match_ref_and_move3();
    pattern_matching_in_tuple_expansion();
    pattern_matching_in_tuple_expansion2();
    match1();
    match2();
    not_ref_defaulting_match();

    // let option_ref = &option;
    // Valid Rust syntax. Used to be required
    // Apparently both & and ref were required
    // match option_ref {
    // Must take reference to Option's inner data - it's a Box and can't be copied
    // &Some(ref i) => println!("Found something: {:?}", i),
    // &None => println!("Found nothing!"),
    // }

    // It was also also valid to
}

fn match_and_move() {
    // The option owns the Box
    let option = Some(Box::new(0));

    match option {
        // This moves the box; it is no longer owned by 'option'
        Some(owns_box) => println!("Found something: {:?}", owns_box),
        None => println!("Found nothing!"),
    }

    // Compiler error - use after move
    // match option {
    // Some(_) => {},
    // None => {},
    // }
}

fn match_and_borrow() {
    // The option owns the Box
    let option = Some(Box::new(0));

    match option {
        Some(ref borrows_box) => println!("Found something: {:?}", borrows_box),
        None => println!("Found nothing!"),
    }

    // Another way to borrow
    match &option {
        Some(borrows_box) => println!("Found something: {:?}", borrows_box),
        None => println!("Found nothing!"),
    }

    // 'option' still owns the Box so we can match again.
    // The previous match only borrowed
    match option {
        Some(_) => {}
        None => {}
    }
}

fn match_ref_and_move1() {
    // Old Rust semantics required a leading '&' during a match
    let option = Some(Box::new(0));
    let option_ref = &option;

    match option_ref {
        &Some(ref borrows_box) => println!("Found something: {:?}", borrows_box),
        &None => println!("Found nothing!"),
    }

    // Box is behind a reference (option_ref) so we can't move it (wy?)
    // match option_ref {
    // &Some(owns_box) => println!("Found something: {:?}", owns_box),
    // &None => println!("Found nothing!"),
    // }
}

fn match_ref_and_move2() {
    // Alternatively, old Rust semantics allow dereferencing during a match
    let option = Some(Box::new(0));
    let option_ref = &option;

    match *option_ref {
        Some(ref owns_box) => println!("Found something: {:?}", owns_box),
        None => println!("Found nothing!"),
    }

    // Can't own/move the value from the option
    // match *option_ref {
    // Some(owns_box) => println!("Found something: {:?}", owns_box),
    // None => println!("Found nothing!"),
    // }
}

fn match_ref_and_move3() {
    // New Rust semantics (match ergonomics) allow a simpler match

    let option = Some(Box::new(0));
    let option_ref = &option;

    // Rust allows auto-dereferencing during pattern-matching.
    match option_ref {
        // option_ref is auto-dereferenced!
        // This is a reference!
        // `borrows_box` is bound line `ref borrows_box`
        Some(borrows_box) => println!("Found something: {:?}", borrows_box),
        None => println!("Found nothing!"),
    }

    // We borrowed before so we can match again
    match option_ref {
        Some(_) => println!("Found something"),
        None => println!("Found nothing!"),
    }
}

fn pattern_matching_in_tuple_expansion() {
    struct TupleStruct(i32);

    // Create a tuple struct
    let tuple_struct = TupleStruct(42);

    // Now destructure the tuple struct
    let TupleStruct(local_var) = tuple_struct;
    println!("local_var: {:?}", local_var);

    // Can do the same with references
    let tuple_struct_ref = &tuple_struct;
    let TupleStruct(local_var) = tuple_struct_ref;
    println!("local_var: {:?}", local_var);

    // In the past you would've had to do
    let tuple_struct_ref = &tuple_struct;
    let TupleStruct(ref local_var) = tuple_struct_ref;
    println!("local_var: {:?}", local_var);
}

fn pattern_matching_in_tuple_expansion2() {
    struct TupleStruct<T>(T);

    // Create a tuple struct
    let tuple_struct = TupleStruct(Box::new(42));

    // Destructure
    // Compiler error
    let TupleStruct(owns_box) = tuple_struct;
    println!("owns_box: {:?}", owns_box);

    // Compiler error - use after move
    // let TupleStruct(owns_box) = tuple_struct;
    // println!("owns_box: {:?}", owns_box);

    //TODO continue
    // let TupleStruct(ref local_var) = tuple_struct;
    // println!("local_var: {:?}", local_var);
    // println!("{:?}", tuple_struct.0);

    // let tuple_struct_ref = &tuple_struct;
}

#[allow(unused)]
fn match1() {
    let x = Some(42);
    let x_ref = &x;

    match x_ref { // <-- matching a reference
        Some(a) => {} // <-- pattern is not a reference
        None => {}
    }
    match x_ref {
        &Some(a) => {},
        &None => {},
    }
    match *x_ref {
        Some(a) => {},
        None => {},
    }
}

#[allow(unused)]
fn match2() {
    let x = Some(Box::new(42));
    let x_ref = &x;

    match x_ref {
        &Some(ref a) => {},
        &None => {},
    }

    match *x_ref {
        Some(ref a) => {},
        None => {},
    }

    match x_ref {
        Some(a) => {
            // x_ref is automatically dereferenced
            // a is automatically bound as a reference
        },
        None => {},
    }
}

#[allow(unused)]
fn not_ref_defaulting_match() {
    let x = Some(Box::new(42));
    let x_ref = &x;

    // The pattern (y) is (a variable binding?) so
    // x_ref is *not* auto-dereferenced.
    // The match uses move semantics (?)
    match x_ref {
        y => {
            // Fails - y is of type &Option<{integer}>
            // if y == 42 {}
        },
    }
}
