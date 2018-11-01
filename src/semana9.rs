use std::io;

const ejercicio : usize = 2;

fn main() {
println!("la respuesta es :{}",SumarMayoresA10(ListaInput()));
}

fn entero(garbage:usize) -> usize {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(i) => return i ,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
   let  mierror : usize = 0;
   return mierror ;
}

fn ListaInput() -> Vec<usize> {
    (0..entero(0)).map(entero).collect()
}

fn SumarMayoresA10(vector :Vec<usize> ) -> usize {
    let diez : usize = 10;
    vector.iter().filter(|x| x >= &&diez ).fold(0,|x,y|x+y)
}