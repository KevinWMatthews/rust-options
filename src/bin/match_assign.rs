/*
Can assign the result of a match.
Both match arms must have the same type!
*/

fn main() {
    println!("\n\tMatching Some");
    let maybe_number = Some(42);
    match_and_assign(maybe_number);

    println!("\n\tMatching None");
    let maybe_number: Option<i32> = None;
    match_and_assign(maybe_number);

    println!("\n\tIf let Some: Some");
    let maybe_number = Some(42);
    if_let_assign_some(maybe_number);

    println!("\n\tIf let Some: None");
    let maybe_number = None;
    if_let_assign_some(maybe_number);

    println!("\n\tIf let None: Some");
    let maybe_number = Some(42);
    if_let_assign_none(maybe_number);

    println!("\n\tIf let None: None");
    let maybe_number = None;
    if_let_assign_none(maybe_number);

    println!("\n\tIf let take: Some");
    let mut maybe_number = Some(42);
    if_let_take(&mut maybe_number);

    println!("\n\tIf let take: now None");
    if_let_take(&mut maybe_number);
}

fn match_and_assign(maybe_number: Option<i32>) {
    dbg!(maybe_number);
    let result = match maybe_number {
        Some(x) => x,
        None => 0,
    };
    println!("Result: {}", result);
}

fn if_let_assign_some(maybe_number: Option<i32>) {
    dbg!(maybe_number);

    // Assign only if the Option contains Some
    if let Some(result) = maybe_number {
        println!("Result: {}", result);
    }
    else {
        println!("Option had nothing");
    }
}

fn if_let_assign_none(maybe_number: Option<i32>) {
    dbg!(maybe_number);

    if let None = maybe_number {
        println!("Option had nothing");
    }
    else {
        println!("Option had something");
    }
}

fn if_let_take(maybe_number: &mut Option<i32>) {
    dbg!(*maybe_number);

    // Option.take() owns the value in an Option and replaces it with None.
    // Can `match` or `if let` on this Option in-place.
    if let Some(result) = maybe_number.take() {
        println!("Took Some: {}", result);
    }
    else {
        println!("Took None");
    }

    match maybe_number {
        Some(_) => panic!("Original Option was taken but still contains something!"),
        None => println!("Original Option is now empty"),
    }
}
