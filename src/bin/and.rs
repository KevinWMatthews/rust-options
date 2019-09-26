/*
Transform a Some into a new Option.
Return a None unchanged.

This allows the caller to modify Some options or change the option type
while preserving None.

The new Option is directly provided by the caller.

Evaluates similar to a logical `and`:
if the first relation is true, return the second relation
if the first relation is false, return the first relation

 A      or   B
--------------------
 Some   or   *Some
 Some   or   *None
*None   or   Some
*None   or   None

https://doc.rust-lang.org/std/option/enum.Option.html#method.and
pub fn and<U>(self, optb: Option<U>) -> Option<U>

For original_option.and(new_option)
If the original Option<T> is Some, return the new Option<U>.
If the original Option<T> is None, return the original Option<U>.
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
