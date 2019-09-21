#![allow(unused)]
/*
Take ownership of a value in an Option and
replace it with None.

https://doc.rust-lang.org/std/option/enum.Option.html#method.take
*/

fn main() {
    // Take from an Option that contains Some
    let mut option = Some(42);
    let new_option = option.take();

    println!("\n\tTaking from an Option: Some");
    print!("Option contains: ");
    print_contents(option);
    print!("New option contains: ");
    print_contents(new_option);

    // Take from an Option that contains None
    let mut option: Option<i32> = None;
    let new_option = option.take();

    println!("\n\tTaking from an Option: None");
    print!("Option contains: ");
    print_contents(option);
    print!("New option contains: ");
    print_contents(new_option);
}

fn print_contents(option: Option<i32>) {
    match option {
        Some(_) => println!("Some"),
        None => println!("None"),
    }
}
