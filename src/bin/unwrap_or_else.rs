/*
Get the value from an Option<T> or a default value.
The default value is returned by a caller-provided closure.
Doesn't panic! Grabs a towel.
Should be safe to use, unlike unwrap().

https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_else
pub fn unwrap_or_else<F>(self, f: F) -> T
where
    F: FnOnce() -> T
*/

fn main() {
    let option = Some(42);
    let value = option.unwrap_or_else(|| {
        0
    });
    println!("value: {}", value);

    let option: Option<i32> = None;
    let value = option.unwrap_or_else(|| {
        0
    });
    println!("value: {}", value);

    let option: Option<i32> = None;
    let value = option.unwrap_or_else(function);
    println!("value: {}", value);
}

fn function() -> i32 {
    0
}
