fn main () {
    let mut a: [i8; 10] = [42; 10];
    //Estructura [T; N] N elementos del tipo T, donde forma parte del tipado.
    a[5] = 0;

    println!("a: {a:?}");

    println!("a: {a:#?}");
    //Mejor formato para lectura
    a[5] = 42;
    println!("Nuevo valor de a: {a:?}");
    let b: [i8; 9] = [42; 9];
    println!("B es un array, de números 42, pero de 9 espacios: {b:?}");

    println!("Tipo de dato a: {}", std::any::type_name_of_val(&a));
    println!("Tipo de dato b: {}", std::any::type_name_of_val(&b));


}