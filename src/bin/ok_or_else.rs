/*
Transform an Option into a Result:
Option<T> -> Result<T, E>
The *value* of the Error is returned by the caller-provided closure.

If Option<T> contains Some(v), simply return Result<v>
If Option<T> contains None, call a user-supplied closure and wrap the result in an Err type

For Option<T>.ok_or(err_closure)
if Option<T> is Some(v), return Ok<v>
if Option<T> is None, return Err(err_closure())

https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else

pub fn ok_or_else<E, F>(self, err: F) -> Result<T, E>
where
    F: FnOnce() -> E
*/

fn main() {
    let option = Some(42);
    let result = option.ok_or_else(|| {
        "error value"
    });
    println!("Result: {:?}", result);

    let option: Option<i32> = None;
    let result = option.ok_or_else(|| {
        "error value"
    });
    println!("Result: {:?}", result);

    let option = Some(42);
    let result = option.ok_or_else(function);
    println!("Result: {:?}", result);

    let option: Option<i32> = None;
    let result = option.ok_or_else(function);
    println!("Result: {:?}", result);
}

// Return type can be anything - it will be wrapped in Err()
fn function() -> &'static str {
    "error value"
}
