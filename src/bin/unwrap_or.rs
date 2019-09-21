/*
Get the value from an Option<T> or a default value.
The default value is directly provided by the caller.
Doesn't panic! Grabs a towel.
Should be safe to use, unlike unwrap().

https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or
pub fn unwrap_or(self, def: T) -> T
*/

fn main() {
    let option = Some(42);
    let value = option.unwrap_or(0);
    println!("value: {}", value);

    let option: Option<i32> = None;
    let value = option.unwrap_or(0);
    println!("value: {}", value);
}
