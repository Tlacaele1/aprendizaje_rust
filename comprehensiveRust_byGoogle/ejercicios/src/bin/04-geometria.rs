// Calcula la magnitud de un vector sumando los cuadrados de sus coordenadas
// y sacando la raíz cuadrada. Usa el método `sqrt()` para calcular la raíz cuadrada
//, como `v.sqrt()`.


fn magnitude(vector: &[f64]) -> f64 {
    let mut suma_cuadrados = 0.0;
    for i in vector {
        suma_cuadrados += i * i;
}
suma_cuadrados.sqrt()
}

// Normaliza un vector calculando su magnitud y dividiendo todas
// sus coordenadas entre esa magnitud.

fn normalize(vector: &mut [f64]) {
    let magnitud = magnitude(&vector);
    for i in 0..vector.len(){
        vector[i] = vector[i] / magnitud;
    }
}

//Solución de Google, apunto diferencias:

// El curso pasa una referencia a un array de tamaño fijo. Mi función puede manejar secuencias de cualquier longitud.
// Quizá la versión del curso sea más segura.
fn magnitude2(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord * coord;
    }
    mag_squared.sqrt()
}

// El curso pasa vector apuntando el tipo y la magnitud del vector que se resolverá.
fn normalize2(vector: &mut [f64; 3]) {
    //A diferencia de mi código, el curso no apunta dos veces al vector. 
    //Mi código apunta dos veces, lo que hace una referencia de la referencia: MEA CULPA.
    let mag = magnitude(vector);
    // El curso accede al valor item con *, mientras que yo lo hice por mediante el índice.
    //Esto es más rápido, pues no se tiene que verificar en cada vuelta que i sea menor al tamaño del vector.
    for item in vector {
        *item /= mag;
    }
}


// Usa `main` para comprobar lo que has hecho.

fn main() {
    println!("Magnitud de un vector unitario: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitud de {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitud de {v:?} después de la normalización: {}", magnitude(&v));

    println!("-------\n SOLUCIÓN DEL CURSO");
    println!("Magnitud de un vector unitario: {}", magnitude2(&[0.0, 1.0, 0.0]));
    let mut v2 = [1.0, 2.0, 9.0];
    println!("Magnitud de {v2:?}: {}", magnitude2(&v2));
    normalize2(&mut v2);
    println!("Magnitud de {v2:?} después de la normalización: {}", magnitude2(&v2));
}