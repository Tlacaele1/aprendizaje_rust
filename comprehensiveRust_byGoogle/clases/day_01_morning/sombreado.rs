fn main() {
    let a = 10;
    println!("antes: {a}");
    {
        let a = "hola";
        println!("ámbito interno: {a}");

        let a = true;
        println!("sombreado en el ámbito interno: {a}");
    }

    println!("después: {a}");
}