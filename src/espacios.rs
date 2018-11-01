fn main() {
    let cadena = "1 2 3".into();
    println!("{}",OtrosCaracteres(cadena));
}

fn NoEspacioRedundante (cadena : String ) -> String {
   let cadena2 : Vec<&str> = cadena.split(" ").filter(|x| x != &"" ).collect();
   return cadena2.join(" ");
}

fn OtrosCaracteres (cadena : String ) -> usize {
    let out : Vec<char> = NoEspacioRedundante(cadena).chars().into_iter().filter(|x| x != &' ').collect();
    out.len()
}

fn OtrosCaracteres2 (cadena : String ) -> usize {
    cadena[0..cadena.len()].into_iter().filter(|x| x != &" " ).collect().len()
}

fn CuentaPalabras (cadena : String ) -> usize {
    let out : Vec<char> = NoEspacioRedundante(cadena).chars().into_iter().filter(|x| x !=  &' ').collect();
    out.len()
}
