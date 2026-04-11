//Una vez habiendo estudiado los vectores en rust, vuelvo al Fibonacci para implementar una solución más eficiente.
use std::io;

fn main() {
    //Declaramos el vector que contendrá los primeros dos índices.
    let mut lista_fibonacci: Vec<i64> = vec![0, 1];
    //Declaramos la variable donde el usuario ingresará el indice deseado y la convertimos a un entero.
    let mut num = String::new();
    println!("Ingrese el índice de Fibonacci que busca: ");
    io::stdin().read_line(&mut num).expect("Error al leer la entrada.");
    let num: i32 = num.trim().parse().expect("Esta función solo acepta números");

    if num == 1 || num == 0 {
        println!(" El índice {num} es {num}"); //Agregamos una verificación por si algún chistosito corre el programa para buscar el índice 0 o 1.
    }else {
        //Sumamos los últimos índices de forma repetida hasta llegar al que buscamos.
        for _ in 1..num {
            lista_fibonacci.push(lista_fibonacci[lista_fibonacci.len()-1] + lista_fibonacci[lista_fibonacci.len()-2]);
        }
        //Reformato del resultado (Se explica abajo)
        let resultado: i64 = lista_fibonacci.last().copied().unwrap_or(0);

        println!("El índice {num} es {resultado}");
    }
}

/*El método .last devuelve un Option<i64>
Necesitamos "quitarle el envoltorio" (por decirlo de alguna forma) para acceder al valor real
Para esto copiamos el valor que se guarda en el último índice con .copied()
luego agregamos .unwrap_or(0) para manejar posibles errores.
Esto lo hacemos porque si pasaramos lista_fibonacci.last() a la función de imprimir nos mostraría Some(RESULTADO)
*/

//Apunte 2: esta nueva implementación aumenta de forma lineal (O(n)) por lo que es mucho más eficiente que 03-Fibonacci.rs