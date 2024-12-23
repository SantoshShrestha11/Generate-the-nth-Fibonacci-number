use std::io;

fn main() {
    println!("Enter the posigion (n) of the Fibonacci number to find: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: usize = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid positive integer.");
            return;
        }
    };
    
    let result = fibonacci_recrusive(n);
    println!("The {n}th Fibonacci number is {result}");

}

fn fibonacci_recrusive(n: usize) -> u64 {
    if n == 0 {
        return 0;
    }
    if n==1 {
        return 1;
    }

    fibonacci_recrusive(n - 1) + fibonacci_recrusive(n-2)
}
