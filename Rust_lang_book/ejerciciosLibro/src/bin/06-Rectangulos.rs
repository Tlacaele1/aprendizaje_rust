fn main () {

    let rectangulo1 = Rectangulo {
        base: 20,
        altura:30,
    };

    let area_calculada = area(&rectangulo1);

    println!("El área de un rectángulo con base {} y altura {} es de: {}", rectangulo1.base, rectangulo1.altura, area_calculada);

}

//Recordar el Upper_camel_case
struct Rectangulo {
    base: i32,
    altura: i32,
}

fn area(rectangulo: &Rectangulo) -> i32 {
    rectangulo.base * rectangulo.altura
}

