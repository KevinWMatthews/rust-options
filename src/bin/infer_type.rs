fn main() {
    // Rust can infer the type of an Option for Some but not for None (there's nothing there)
    let thing = Some("This is a string");
    dbg!(thing);

    let thing = Some(123);
    dbg!(thing);

    // Compiler error:
    // cannot infer type for `T`
    /*
    let thing = None;
    */

    // Must specify type if creating a None variant of an Option
    let thing: Option<i32> = None;
    dbg!(thing);
}
