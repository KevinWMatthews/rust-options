fn main() {
    /* Options are an enum variant:
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
}
