fn main() {
    println!("* * * * Fibonacci Sequence * * * *");
    println!("{}", generate_fibonacci(19));
}

fn generate_fibonacci(n: i32) -> i32 {
    let result;
    if n <= 2 {
        result = 1;
    } else {
        let n1 = generate_fibonacci(n - 1);
        let n2 = generate_fibonacci(n - 2);
        result = n1 + n2;
    }
    result
}
