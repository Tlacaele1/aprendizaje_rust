#[rustfmt::skip]
fn main() {
    let input = 'x';
    match input {
        'q'                       => println!("Salir"),
// operador lógico OR
        'a' | 's' | 'w' | 'd'     => println!("Desplazarse"),
// .. representa un rango
        '0'..='9'                 => println!("Introducción de números"),
        key if key.is_lowercase() => println!("Minúscula: {key}"),
// _ Se usa para atrapar todas las excepciones, es como un else.
        _                         => println!("Otro"),
    }
}