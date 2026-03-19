#[derive(Debug)]
/// Un evento en el sistema de ascensores al que debe reaccionar el controlador.
enum Event {
    // TAREAS: añadir variantes obligatorias
    Llamada(Floor, Direction),
    BotonInterior(Floor),
    AbrirPuerta,
    CerrarPuerta,
    Llegada(Floor)
}

//Creamos el tipo Floor porque se usa como argumento.
type Floor = i32;

/// Un sentido de la marcha.
#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

/// El ascensor ha llegado a la planta indicada.
fn car_arrived(floor: i32) -> Event {
    Event::Llegada(floor)
}

/// Las puertas del ascensor se han abierto.
fn car_door_opened() -> Event {
    Event::AbrirPuerta
}

/// Las puertas del ascensor se han cerrado.
fn car_door_closed() -> Event {
    Event::CerrarPuerta
}

/// Se ha pulsado el botón direccional de un ascensor en la planta indicada.
fn lobby_call_button_pressed(floor: i32, dir: Direction) -> Event {
    Event::Llamada(floor, dir)
}

/// Se ha pulsado el botón de una planta en el ascensor.
fn car_floor_button_pressed(floor: i32) -> Event {
    Event::BotonInterior(floor)
}

fn main() {
    println!(
        "Un pasajero de la planta baja ha pulsado el botón para ir hacia arriba: {:?}",
        lobby_call_button_pressed(0, Direction::Up)
    );
    println!("El ascensor ha llegado a la planta baja: {:?}", car_arrived(0));
    println!("Las puertas del ascensor se han abierto: {:?}", car_door_opened());
    println!(
        "Un pasajero ha pulsado el botón de la tercera planta: {:?}",
        car_floor_button_pressed(3)
    );
    println!("Las puertas del ascensor se han cerrado: {:?}", car_door_closed());
    println!("El ascensor ha llegado a la tercera planta: {:?}", car_arrived(3));
}