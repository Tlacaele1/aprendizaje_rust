fn main() {

    //El curso usa un array mutable, pero el compilador recomienda eliminar la mutabilidad.
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");
    //El slice nos permite tomar una "rebanada" del array
    let s: &[i32] = &a[2..4];

    println!("s: {s:?}");
    //Podemos usar len() para tomar hasta el último índice
    let t: &[i32] = &a[3..a.len()];
    println!("t: {t:?}");

    //Se puede crear un slice del array completo
    let u: &[i32] = &a[..];
    println!("u: {u:?}");
}