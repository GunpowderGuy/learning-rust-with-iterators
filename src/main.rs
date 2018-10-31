fn main() {
    println!("Hello, world!");
    println!("{}",numeroPerfecto(8128));
    println!("{}",CuentaPalabras("uno  dos      tres"));
    println!("{}", OtrosCaracteres("uno1  2"))
    }

fn factorial(n : usize) -> usize {
    (1..n+1).fold(1,|x,y|x*y)
}

fn factor(n:usize) -> usize {
    (1..n+1).map(factorial).fold(0,|x,y|x+y)
}

fn numeroPerfecto(n:i64) -> bool {
    n == (1..n).filter(|x|n%x == 0).fold(0,|x,y|x+y)
}

fn SepararPalabras (cadena : &str ) -> Vec<&str> {
    cadena.split(" ").filter(|x| x != &"" ).collect()
}

fn OtrosCaracteres (cadena : &str ) -> usize {
    SepararPalabras(cadena).join("").len()
}

fn CuentaPalabras (cadena : &str) -> usize {
    SepararPalabras(cadena).len()
}
