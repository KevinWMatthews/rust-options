/*
Return the value of an Option or a default for the given type.

Should be safe to use, unlike unwrap().

Can be used with types that implement the Default trait:
https://doc.rust-lang.org/std/default/trait.Default.html#tymethod.default

Options do implement Default; they return None.

https://doc.rust-lang.org/std/option/enum.Option.html#method.unwrap_or_default
pub fn unwrap_or_default(self) -> T
*/

fn main() {
    let option = Some(42);
    let value = option.unwrap_or_default();
    println!("value: {}", value);

    // Options return None via the Default trait
    let option: Option<i32> = None;
    let value = option.unwrap_or_default();
    println!("value: {}", value);
}
