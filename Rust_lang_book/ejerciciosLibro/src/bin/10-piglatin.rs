//Convierte strings a pig latin. La primera consonante de cada palabra se mueve al final de la palabra y se agrega "ay", 
//por lo que "primero" se convierte en "rimepay". 
//Sin embargo, si la palabra comienza con una vocal, 
//simplemente agregue "hay" al final de la palabra ("manzanaay"). ¡Ten en cuenta las reglas de UTF-8!
use std::io;

fn main() {
    let mut frase = String::new();
    println!("Escribe aquello que quieres traducir a pig latin: ");
    io::stdin().read_line(&mut frase).expect("Error al leer la entrada");
    let lista = pig_latin(&frase);
    println!("{:#?}", lista); 

}

fn pig_latin(palabra: &String) -> Vec<String> {
    let mut lista_de_palabras: Vec<String> = Vec::new();
    let vocales = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    for word in palabra.split_whitespace() {
        let mut chars = word.chars();
            if let Some(primero) = chars.next() {   //obtenemos el primer caracter, si existe se guarda en "primero" .Ojo, "chars" pierde el primer caracter
                let nueva_palabra = if vocales.contains(&primero){ //el valor de nueva palabra dependerá de si comienza con vocal
                    format!("{}hay", word) //Si comienza con vocal, se ejecuta esto y es asignado al valor de nueva_palabra.
                } else {
                    let resto: String = chars.collect(); //Si no comienza con vocal, se ejecuta esto. Se toma el todo lo que no es "primero" y se asigna a "resto"
                    format!("{}{}ay", resto, primero) //luego (resto, primero "ay") se asignan en ese orden a nueva_palabra.
                };
                lista_de_palabras.push(nueva_palabra); //Añadimos nueva_palabra al arreglo lista_de_palabras
            }

    }
    lista_de_palabras

}