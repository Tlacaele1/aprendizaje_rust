enum Result {
    Ok(i32),
    Err(String),
}

fn divide_in_two(n: i32) -> Result {
    if n % 2 == 0 {
        Result::Ok(n / 2)
    } else {
        Result::Err(format!("no se puede dividir {n} en dos partes iguales"))
    }
}

//Probar con números impares para imprimir el mensaje de error.
fn main() {
    let n = 20;
    match divide_in_two(n) {
        Result::Ok(half) => println!("{n} dividido entre dos es {half}"),
        Result::Err(msg) => println!("se ha producido un error: {msg}"),
    }
}