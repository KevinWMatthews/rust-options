/*
Get the value from an Option<T> or panic!

*Not recommended* unless you don't mind your program crashing.

If Option<T> contains Some(v), return v
if Option<T> contains none, panic!

https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap
pub fn unwrap(self) -> T
*/

fn main() {
    let option = Some(42);
    let value = option.unwrap();
    println!("value: {}", value);

    let option: Option<i32> = None;
    let value = option.unwrap();
    println!("value: {}", value);
}
