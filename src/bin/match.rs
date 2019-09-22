fn main() {
    println!("\n\tMatching Some");
    let maybe_number: Option<i32> = Some(42);
    match_and_use(maybe_number);

    println!("\n\tMatching None");
    let maybe_number: Option<i32> = None;
    match_and_use(maybe_number);

    println!("\n\tMatch and ignore");
    let maybe_number: Option<i32> = Some(7);
    match_and_ignore(maybe_number);
}

fn match_and_use(maybe_number: Option<i32>) {
    dbg!(maybe_number);
    match maybe_number {
        // Gets the value from Some, stores it in x
        Some(x) => println!("Matched Some: {}", x),
        None => println!("Matched None"),
    }
}

fn match_and_ignore(maybe_number: Option<i32>) {
    dbg!(maybe_number);
    match maybe_number {
        // Ignores the value in Some
        Some(_) => println!("Matched Some"),
        None => println!("Matched None"),
    }
}
