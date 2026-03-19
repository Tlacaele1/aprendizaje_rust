
//Definición de la estructura
struct Person {
    name: String,
    age: u8,
}

// La función recibe person, y el tipo de dato de person apunta a la estructura
fn describe(person: &Person) {
    println!("{} tiene {} años", person.name, person.age);
}

fn main() {
    let mut peter = Person { name: String::from("Peter"), age: 27 };
    //se apunta a peter para pasarlo como argumento
    describe(&peter);

    peter.age = 28;
    describe(&peter);

    //Otra forma de definir: primero las características y luego estas pasan como argumento a avery = Person {}
    let name = String::from("Avery");
    let age = 39;
    let avery = Person { name, age };
    //Ojo aquí, los argumentos pasan como apuntadores.
    describe(&avery);

    //age de jackie es copiada de avery mediante el uso de los dos puntos.
    let jackie = Person { name: String::from("Jackie"), ..avery };
    describe(&jackie);
}