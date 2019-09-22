/*
Transform an Option if it is None.
Return the Option otherwise.
The new Option is directly provided by the caller.

If Option<T> contains Some(v), simply return Some(v)
If Option<T> contains None, return the caller-provided Option

For Option<T>.or(opt)
if Option<T> is Some(v), return Some(v)
if Option<T> is None, return opt

Returns:
if Some, the original Option
if None, the given Option

Returns the original Option (if Some) *or* the given Option.

https://doc.rust-lang.org/std/option/enum.Option.html#method.or
*/


fn main() {
    // with Some
    let option = Some(42);
    let other = Some(0);
    // let other: Option<i32> = None;

    let new_option = option.or(other);
    dbg!(new_option);

    // with None
    let option = None;
    let other = Some(0);
    let new_option = option.or(other);
    dbg!(new_option);
}
