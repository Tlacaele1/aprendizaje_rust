
fn main(){
    let s = String::from("¡Quihubo, la bandera!");

    //Esta variable guarda el último índice de la palabra antes del espacio en blanco
    let word = first_word(&s);

    //Accederemos a la primera palabra a través de un slice desde el índice 0 hasta el valor de "word"
    let primera =&s[0..word];

    println!("la primera palabra es {}", primera);

    let palabra = primera_palabra(&s);
    
    println!("{palabra}");

}

fn first_word(s: &String) -> usize {
    //Esta funcion será un iterador sobre la string s (convertida en bytes)
    let bytes = s.as_bytes();
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
    s.len()
}

//Otra forma, más directa:
fn primera_palabra(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}