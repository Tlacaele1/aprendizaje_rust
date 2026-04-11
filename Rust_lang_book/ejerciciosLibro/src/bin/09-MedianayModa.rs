//Dada una lista de enteros, usa un vector y devuelve la mediana 
//(cuando se ordena, el valor en la posición media) y la moda 
//(el valor que ocurre con más frecuencia; un hash map será útil aquí) de la lista.
use std::collections::HashMap;


fn main(){
    // Con un script generamos 101 números aleatorios entre 0 y 53
    let mut list = vec![9, 41, 24, 48, 18, 8, 23, 18, 30, 5, 50, 41, 20, 20, 4, 32, 26, 20, 21, 34, 12, 41, 23, 28, 46, 34, 17, 12, 18, 28, 48, 5, 6, 15, 39, 31, 2, 24, 13, 27, 39, 28, 12, 35, 3, 13, 36, 14, 5, 27, 23, 41, 16, 53, 6, 18, 37, 3, 40, 50, 47, 8, 28, 8, 17, 16, 43, 22, 18, 5, 1, 18, 6, 51, 13, 10, 32, 30, 37, 11, 20, 49, 52, 36, 52, 42, 10, 5, 46, 13, 31, 12, 48, 34, 5, 34, 30, 48, 0, 4, 34];
    list.sort();

    let mediana = mediana(&list);
    let moda = moda(&list);

    println!("La mediana es {}, y la moda {}", mediana, moda);
}

fn mediana(lista: &Vec<i32>) -> i32 {
    lista[lista.len() / 2]
}

fn moda(lista: &Vec<i32>) -> i32 {
    let mut moda = HashMap::new();
    for &valor in lista{
        let count = moda.entry(valor).or_insert(0);
        *count += 1;
    }
    moda.into_iter().max_by_key(|&(val, count)| (count, -val)).map(|(val, _)| val).unwrap_or(0)

}


