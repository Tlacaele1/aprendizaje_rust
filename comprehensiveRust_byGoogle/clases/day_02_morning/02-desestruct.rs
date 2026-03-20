struct Foo {
    x: (u32, u32),
    y: u32,
}

//las estructuras se pueden desestructurar con la coincidencia.
// probar con los casos contemplados: cambiar el 1 en x y añadir un 2 en y; quitar todas las coincidencias en x y y.
#[rustfmt::skip]
fn main() {
    let foo = Foo { x: (1, 2), y: 3 };
    match foo {
        Foo { x: (1, b), y } => println!("x.0 = 1, b = {b}, y = {y}"),
        Foo { y: 2, x: i }   => println!("y = 2, x = {i:?}"),
        Foo { y, .. }        => println!("y = {y}, se han ignorado otros campos"),
    }
}