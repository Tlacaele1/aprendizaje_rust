
// Se declara un arreglo DIGEST_SIZE de tres posiciones
const DIGEST_SIZE: usize = 3;
// Zero representa un valor que puede estar presente o ausente. En este caso existe y es 42
const ZERO: Option<u8> = Some(42);

// Esta función recibe una referencia de un texto y devuelve el arreglo de tres posiciones DIGEST_SIZE
fn compute_digest(text: &str) -> [u8; DIGEST_SIZE] {
//Se declara digest como un arreglo cuya extensión es DIGEST_SIZE(3) y los valores son tomados de ZERO. En caso de que no haya valores en ZERO, los valores serían 0.
    let mut digest = [ZERO.unwrap_or(0); DIGEST_SIZE];
//Se hace una iteración sobre los elementos de text convertidos en bytes
    for (idx, &b) in text.as_bytes().iter().enumerate() {
// idx = 0, 1 ,2 porque sólo hay tres índices en el arreglo DIGEST_SIZE.
//0%3=0, 1%3=1, 2%3=2, 3%3=0, 4%3=1, EVITA el desbordamiento. Recordar que % es el operador de módulo
//.wrapping_add suma b (El valor en ASCII del texto) a la posición determinada por la operación antes explicada. Si pasa de 255 regresa a 0 y sigue sumando.
        digest[idx % DIGEST_SIZE] = digest[idx % DIGEST_SIZE].wrapping_add(b);
    }
//El algoritmo devuelve digest, con las tres posiciones constantes que se declararon en un principio.
    digest
}

fn main() {
    let digest = compute_digest("Hello");
    println!("digest: {digest:?}");
}