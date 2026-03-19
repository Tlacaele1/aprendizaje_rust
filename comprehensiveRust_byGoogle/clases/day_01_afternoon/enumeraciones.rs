//Guardan un conjunto de valores en un solo tipo. Estas pueden tener variantes.

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
enum PlayerMove {
    Pass,                        // Variante simple
    Run(Direction),              // Variante de tupla
    Teleport { x: u32, y: u32 }, // Variante de struct
}

fn main() {
    let m: PlayerMove = PlayerMove::Run(Direction::Left);
    println!("En este turno: {:?}", m);
    let n : PlayerMove = PlayerMove::Teleport{x:10, y:5};
    println!("En este turno: {:?}", n);
}