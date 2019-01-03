fn main() {
    println!("");

    // Complete use of Option type.
    let number: Option<i32> = Some(5);
    println!("number: {:?}", number);

    // Compiler will auto-detect the type
    let string = Some("some string");
    println!("string: {:?}", string);

    // Compiler can not auto-detect type of None
    let nothing: Option<u32> = None;
    println!("nothing: {:?}", nothing);
}
