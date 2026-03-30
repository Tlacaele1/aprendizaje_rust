//Honor a quien honor merece:
//Este programa incluye una pequeña traducción del programa homónimo propuesto por Kernighan y Ritchie en EL LENGUAJE DE PROGRAMACIÓN C
//A su vez, también incluye un loop que pregunta al usuario si desea convertir una temperatura específica de Celsius a Farenheit.
//La lógica es de Celsius a Fahrenheit porque soy latino, y acá usamos Celsius.
use std::io;

const LOWER: f64 = 0.0;
const UPPER: f64 = 300.0;
const STEP: f64 = 20.0;


fn main () {
    //Homenaje
    let mut cel= LOWER;
    println!("Celsius\tFahrenheit\n");
        while cel <= UPPER{
            let fahr = cel*(9.0/5.0) + 32.0;
            println!("{}\t{}", cel, fahr);
            cel += STEP;
        }

    //loop interactivo
    loop {
        let mut temp: String= String::new();
        println!("¿Desea convertir una temperatura específica?");
        println!("Ingrese el valor para hacerlo, de lo contrario ingrese n");
        io::stdin().read_line( &mut temp).expect("Error al leer la entrada");
        if temp.trim() == "n"{break;}
        let temp: f64 = match temp.trim().parse() {
            Ok(temp)=> temp,
            Err(_) => continue,
        };
        let conversion = temp*(9.0/5.0)+ 32.0;
        println!("{temp} grados celsius son {conversion} grados fahrenheit");
    }
}