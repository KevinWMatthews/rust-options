/*
Transform an Option<T> into an Option<U> if it is Some.
Return None otherwise.
The new Option is directly returned by a caller-provided closure.

The closure is passed the current value of the Option.

https://doc.rust-lang.org/std/option/enum.Option.html#method.and_then
pub fn and_then<U, F>(self, f: F) -> Option<U>
where
    F: FnOnce(T) -> Option<U>,
*/

fn main() {
    let option = Some(0);
    let new_option = option.and_then(|_| { Some(42) });
    dbg!(new_option);

    let option: Option<i32> = None;
    let new_option = option.and_then(|_| { Some(42) });
    dbg!(new_option);
}
