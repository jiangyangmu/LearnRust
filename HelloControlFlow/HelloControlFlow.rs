fn main() {
	let mut a = 0;
	if true {
		a += 10;
	} else {
		a += 20;
	}

	let mut b = 0;
	loop {
		if b < 10 {
			b += 1;
		} else {
			break;
		}
	}

	let mut c = 0;
	while c < 10 {
		c += 1;
	}

	let mut d = 0;
	for i in 0..5 {
		d += i;
	}

	println!("if-else = {}", a);
	println!("loop = {}", b);
	println!("while = {}", c);
	println!("for = {}", d);
}