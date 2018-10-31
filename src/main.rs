fn main() {
    println!("Hello, world!");
    println!("{}",numeroPerfecto(8128))
}

fn factorial(n : i64) -> i64 {
    (1..n+1).fold(1,|x,y|x*y)
}

fn factor(n:i64) ->i64 {
    (1..n+1).map(factorial).fold(0,|x,y|x+y)
}

fn numeroPerfecto(n:i64) -> bool {
    n == (1..n).filter(|x|n%x == 0).fold(0,|x,y|x+y)
}