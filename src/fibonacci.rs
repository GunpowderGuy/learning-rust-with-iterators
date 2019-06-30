fn fib(n: usize) -> usize {
    match n {
        0 => 1,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
  println!("resultado del fibonacci es {fibo}",
   fibo = fib(3));
   
   println!();
}
