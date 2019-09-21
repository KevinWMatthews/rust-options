/*
Transform an Option into an Result:
Option<T> -> Result<T, E>
The *value* of the error is provided by the caller.

If Option<T> contains Some<v>, simply return Result<v>
If Option<T> contains None, wrap a user-provided `err` value in an Err type

For Option.ok_or(err_value)
if Option<T> is Some<v>, return Ok(v)
if Option<T> is None, return Err(err_value)

Note that Rust wraps the argument err in the Err type.

https://doc.rust-lang.org/std/option/enum.Option.html#method.ok_or_else
pub fn ok_or<E>(self, err: E) -> Result<T, E>
*/

fn main() {
    let option = Some(42);
    let result = option.ok_or("Error value");
    println!("Result: {:?}", result);

    let option: Option<i32> = None;
    let result = option.ok_or("Error value");
    println!("Result: {:?}", result);
}
