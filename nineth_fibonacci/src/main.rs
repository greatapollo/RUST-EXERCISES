fn fibonacci(n: u32) ->u32 {
    if n <= 1 {
           return n; 
        }else {
            return fibonacci(n-1) + fibonacci(n-2);
        }
    }

fn main(){
    let n = 9;
    let fib_number: u32 = fibonacci(n);
    println!("The {}th Fibonacci number is: {}", n, fib_number);
} 



