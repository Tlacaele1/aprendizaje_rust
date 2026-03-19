fn main() {
    let a = 'A';
    let b = 'B';
    //r hace referencia al valor de A
    let mut r: &char = &a;
    //el operador * "cambia" la referencia por el valor de real de a
    println!("r: {}", *r);
    r = &b;
    println!("r: {}", *r);

    // En rust una referencia no permite modificar el valor de la variable, aunque la referencia sea mutable.
    //probar *r = 'X' , el programa no compilará
}