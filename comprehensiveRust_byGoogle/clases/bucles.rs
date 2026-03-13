fn main () {
	let mut x = 200;
	while x >= 10 {
		x = x / 2;
	}
	println!("x final : {x}");

	for i in 1..5{
		println!("i: {i}");
	}
	
	for elem in [1, 2, 3, 4, 5] {
		println!("elem: {elem}");
	}

	let mut y = 0;
	loop {
		y+=1;
		println!("{y}");
		if y > 100 {
			break;
		}
	}

}


