// Solution: Our function takes a 32bit signed integer as input, performs math
// and returns celsius as 32bit signed integer. We could do floats to be more
// accurate, but I'm not really concerned here.

// Rust gives us a few options re: dispatch so we have a lot of control here.

// For both solutions, we pass in a closure as an argument. Acc. to docs:
// 'closures are traits.' A trait is 'a collection of methods defined for an
// unknown type: Self'

// Static dispatch approach: According to docs, 'Rust favors this.' We assign
// behavior at compile time. Done through 'monomorphization.' The 'where' clause
// is how we create a bound for our type, here it's F.

fn convert<F>(measurement: i32, scale_system: F) -> i32      where F: Fn(i32) ->
i32 {

    scale_system(measurement) }

// Dynamic dispatch approach: Rust can also do this. We assign behavior at
// runtime. Here we reference the trait with &. See comment below on using error
// messages to reveal this.

// fn convert(measurement: i32, scale_system: &Fn(i32) -> i32) -> i32 {
// scale_system(measurement) }

fn main() {     // Pass in either lambda/closure to the converting function let
to_celsius = |temp: i32| -> i32 { (temp - 32) * 5/9 };

    let to_fahrenheit = |temp: i32| -> i32 { (temp * 9/5) + 32 };

    // We pass &closure. Passing to_celsius (no ampersand) results in an error
msg: 'expected reference, found closure'. It expects us to reference the closure
not pass it in as an arg. We could pass in the closure not binded, as a literal
and it would compile. println!("{} degrees Fahrenheit is {} Celsius", 85,
convert(85, &to_celsius));     println!("{} degrees Celsius is {} Fahrenheit",
29, convert(29, &to_fahrenheit)); }
