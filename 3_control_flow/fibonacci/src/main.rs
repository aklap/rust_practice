fn main() {      
    // I chose to begin my sequence with 0. Sometimes people choose to begin
    // with 1, though. We could have another optional argument for start value
    // of fib sequence.

    fn get_nth(n: i32) -> i32 { 
        if n == 1 { 
            0 
        } else if n == 2 { 
            1 
        } else {
            get_nth(n-1) + get_nth(n-2) 
        } 
    }

    // Generate the nth Fibonacci number 
    //let fib = get_nth(5);
    
    println!("{}", fib); 
}

