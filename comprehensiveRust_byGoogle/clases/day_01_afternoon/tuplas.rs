fn main() {
    //Permiten guardar tipos de datos distintos. Se accede a los datos con nombredelatupla.índice
    let t: (i8, bool) = (7, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);

    print_tuple(t);

    print_tuple2(t);
}

//Patrones y Desestructuración

fn print_tuple(tuple: (i8, bool)) {
    let left = tuple.0;
    let right = tuple.1;
    println!("left: {left}, right: {right}");
}

fn print_tuple2(tuple: (i8, bool)) {
    let (left, right) = tuple;
    println!("left: {left}, right: {right}");
}