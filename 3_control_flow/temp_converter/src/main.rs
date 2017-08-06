// Our function that takes a 32bit signed integer as input, performs math and returns celsius as 32bit signed integer
fn converter(temp: i32) -> i32 {
    // Bind to variable our math operations with input, 'converted' an immutable variable
    let converted = (temp - 32) * 5/9;
    // Implicit return, no semicolon here bc it's an expression not statment
    converted
}

fn main() {
    // Test our function works, we expect 29
    let celsius = converter(85);

    // Print with value of variable we binded using string interpolation 
    println!("In Celsius that is {}", celsius);
}