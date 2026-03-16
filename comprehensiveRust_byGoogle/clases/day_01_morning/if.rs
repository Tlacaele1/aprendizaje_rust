fn main (){
	let x=10;
	if x == 0 {
		println!("¡Cero!");
	} else if x < 100 {
		println!("Muy grande");
	} else {
		println!("Enorme");
	}
	

	let y = 10;
	let size = if y < 20 {"pequeño"} else {"grande"};
	println!("Tamaño del número: {}", size);
}
