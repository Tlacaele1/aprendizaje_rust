
// Las variables estáticas vivirán durante toda la ejecución del programa y, por lo tanto, no se moverán.

static BANNER: &str = "Bienvenide a RustOS 3.14";

fn main() {
    println!("{BANNER}");
}

//Estas no son insertadas y tienen una ubicación de memoria real asociada.