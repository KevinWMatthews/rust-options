/*
Transform a None into a new Option.
Return a Some unchanged.

This allows the caller to suppress or modify None options while preserving Some values.

Note that the type of Some can not be changed.

The new Option is directly provided by the caller.

Evaluates similar to a logical or:
if the first relation is true, return the first relation.
if the first relation is false, return the second relation.

 A      or   B
-----------------
*Some   or   Some
*Some   or   None
 None   or  *Some
 None   or  *None

https://doc.rust-lang.org/std/option/enum.Option.html#method.or
pub fn or(self, optb: Option<T>) -> Option<T>

For original_option.or(new_option)
if the original Option<T> is Some, return the original Option<T>.
if the original Option<T> is None, return the new Option<T>.
*/


fn main() {
    // with Some
    let option = Some(42);
    let other = Some(0);
    // let other: Option<i32> = None;

    let new_option = option.or(other);
    dbg!(option);
    dbg!(new_option);

    // with None
    let option = None;
    let other = Some(0);
    let new_option = option.or(other);
    dbg!(new_option);
}
