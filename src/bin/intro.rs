fn main() {
    /*
    Options are an enum variant:
    enum Option<T> {
        Some<T>,
        None,
    }
    They either contain:
    a generic type, T
    nothing
    */
    let maybe_number: Option<i32> = Some(42);
    dbg!(maybe_number);

    let maybe_number: Option<i32> = None;
    dbg!(maybe_number);

    // Types can be inferred for most Some()
    let maybe_number = Some(4);
    dbg!(maybe_number);

    // Types cannot be inferred for None - there is nothing there!
    // Compiler error:
    // cannot infer type for `T`
    /*
    let maybe_number = None;
    */
    let maybe_number: Option<i32> = None;
    dbg!(maybe_number);
}
