use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Adivina el número.");
    //Generar un número aleatorio
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop{
        println!("Escribe tu suposición: ");

        //Declaramos la variable que guardará la entrada del usuario.
        let mut guess = String::new();

        //llamada a la biblioteca.método específico. manejo de errores
        io::stdin().read_line(&mut guess).expect("Error al leer la entrada.");
        //Reformato de guess
        let guess: u32 = guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Tu suposición es {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muy pequeño"),
            Ordering::Greater => println!("Muy grande"),
            Ordering::Equal => {
                println!("¡Ganaste!");
                break;
            }
        }
    }
}
