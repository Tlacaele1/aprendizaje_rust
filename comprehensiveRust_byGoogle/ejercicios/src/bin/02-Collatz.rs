/// Determina la longitud de la secuencia de Collatz que empieza por "n"
fn main(){
    println!("Longitud para n=11: {}", collatz_lenght(11));
    println!("Longitud con la solución de Google: {}", google_collatz_length(11));
}

//Mi solución
fn collatz_lenght(mut n: i32) -> u32 {
    let mut counter = 1;
    while n != 1{
        counter += 1;
        if n % 2 == 0 {
            n= n / 2;
        }else{
            n = (3 * n) + 1;
        }
    }
    return counter;
}

// Solución de Google
fn google_collatz_length(mut n: i32) -> u32 {
    let mut len = 1;
    while n > 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        len += 1;
    }
    len
}