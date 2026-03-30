use std::io;

fn main(){
    let mut num = String::new();
    println!("Ingresa el índice de Fibonacci que deseas: ");
    io::stdin().read_line(&mut num).expect("Error al leer la entrada");
    let num: u32= num.trim().parse().expect("Escriba sólo números");
    println!("Indíce {num} = {}", fibo(num));
}

fn fibo(num: u32) -> u32 {
    if num < 2 {
        return num;
    } else {
        return fibo(num-1) + fibo(num-2);
    }
}

// Este programa explota con num > 40. (STACK OVERFLOW)
//Su rendimiento es O(2^n), debido a la recursividad.
//Esta es una solución tomada del curso de Google,
//Volveremos a este problema más adelante en el estudio, con la intención de hacer una solución de crecimiento lineal O(n).