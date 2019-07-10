// https://doc.rust-lang.org/std/option/index.html
// https://doc.rust-lang.org/std/option/enum.Option.html

fn main() {
    enum_variants();
    auto_detect_type();
    matching_options();
}

fn enum_variants() {
    // Option is a enum with two variants: Some(T) and None
    println!("\n\tEnum variants");

    let number: Option<i32> = Some(5);
    println!("number: {:?}", number);

    let number: Option<i32> = None;
    println!("number: {:?}", number);
}

fn auto_detect_type() {
    // The compiler can auto-detect the type of most options
    println!("\n\tAuto-detect type");

    let string = Some("this is a string");
    println!("string: {:?}", string);

    let number = Some(1234);
    println!("number: {:?}", number);

    // The compiler can not auto-detect the type of None
    // COmpiler error: cannot infer type for `T`
    // let nothing = None;
    let nothing: Option<u32> = None;
    println!("nothing: {:?}", nothing);
}

fn matching_options() {
    println!("\n\tMatching options");

    // Match None type
    let maybe_number: Option<i32> = None;
    match maybe_number {
        None => println!("Nothing there!"),
        Some(i) => println!("Found something: {}", i),
    }

    // Match and get the specific value from the Option
    let maybe_number = Some(5);
    match maybe_number {
        None => println!("Nothing there!"),
        Some(i) => println!("Found something specific: {}", i),
    }

    // Match where we don't care about the value in the Option
    let maybe_number = Some(5);
    match maybe_number {
        None => println!("Nothing there!"),
        Some(_) => println!("Found something."),
    }
}
