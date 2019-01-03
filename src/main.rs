fn main() {
    println!("\n\tBasic options");

    // Complete use of Option type.
    let number: Option<i32> = Some(5);
    println!("number: {:?}", number);

    // Compiler will auto-detect the type
    let string = Some("some string");
    println!("string: {:?}", string);

    // Compiler can not auto-detect type of None
    let nothing: Option<u32> = None;
    println!("nothing: {:?}", nothing);

    println!("\n\tMatching options");
    let five = Some(5);
    let six = plus_one(five);
    println!("six: {:?}", six);

    let none = plus_one(None);
    println!("none: {:?}", none);
}

fn plus_one(number: Option<i32>) -> Option<i32> {
    match number {
        None => None,
        Some(i) => Some(i + 1),
    }
}
