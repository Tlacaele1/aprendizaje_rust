fn main() {
    //&str es una vista inmutable y prestada de una cadena. Es un par: puntero, longitud.
    let s1: &str = "mundo";
    println!("s1: {s1}");
    
    //String es un tipo de cadena propietario, mutable y almacenado en el heap (crece dinámicamente).
    let mut s2: String = String::from("¡Hola ");
    println!("s2: {s2}");
    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[s2.len() - s1.len()..];
    println!("s3: {s3}");
}