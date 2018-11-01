const ejercicio : usize = 2;

fn main() {

println!("  fef");
}

fn entero() -> usize {
let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => return i ,
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
}

fn ListaInput() -> Vec<usize> {
    (0..entero()).map(entero).collect()
}

fn SumarMayoresA10(vector :Vec<usize> ) -> usize {
    vector.iter().fold(0,|x,y|x+y)
}