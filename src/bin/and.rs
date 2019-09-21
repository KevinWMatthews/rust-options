/*
Transform an Option if it is Some.
Return None otherwise.
The new Option is directly provided by the caller.

Unlike or(), and() returns the *new* option.
There is no need to return Some/None for the original option; we already have that.

If Option<T> contains Some<v>, return the caller-provided Option
if Option<T> contains None, return None

For Option<T>.and(opt)
if Option<T> is Some<v> return opt
if Option<T> is None, return None

https://doc.rust-lang.org/std/option/enum.Option.html#method.and
pub fn and<U>(self, optb: Option<U>) -> Option<U>
*/

fn main() {
    // With Some
    let option = Some(42);
    let replacement = Some(0);
    let new_option = option.and(replacement);
    dbg!(new_option);

    // With None
    let option: Option<i32> = None;
    let replacement = Some(0);
    let new_option = option.and(replacement);
    dbg!(new_option);
}
