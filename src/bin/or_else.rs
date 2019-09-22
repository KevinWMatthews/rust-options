/*
Transform an Option into a different Option if it is None.
Return the Option otherwise.
The new Option is directly returned by the caller-provided closure.

If Option<T> contains Some(v), simply return Some(v)
If Option<T> contains None, call the closure and return its result

For Option<T>.or_else(closure)
if Option<T> is Some(v), return Some(v)
if Option<T> is None, return closure()

The closure must:
accept no arguments
return Option<T>

https://doc.rust-lang.org/std/option/enum.Option.html#method.or_else
pub fn or_else<F>(self, f: F) -> Option<T>
where
    F: FnOnce() -> Option<T>
*/

fn main() {
    let option = Some(42);
    let new_option = option.or_else(function);
    dbg!(new_option);

    let option: Option<i32> = None;
    let new_option = option.or_else(function);
    dbg!(new_option);

    // Can pass a closure instead of a function
    let option: Option<i32> = None;
    let new_option = option.or_else(|| {
        Some(-42)
    });
    dbg!(new_option);
}

// Return type must be Option<i32>
fn function() -> Option<i32> {
    Some(-42)
    // None
}
