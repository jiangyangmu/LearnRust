fn fun_move(s : String) -> String {
	s
}
fn fun_ref(s : &String) -> &String {
	s
}
fn fun_refmut(s : &mut String) -> &String {
	s.push_str(" World");
	s
}

fn main() {
	// Move by default

	let movesrc1 = String::from("Hello");
	let movesrc2 = String::from("Hello");

	let varmove = movesrc1;
	let varclone = varmove.clone();
	let parammove = fun_move(movesrc2);

	println!("var move by default = {}", varmove);
	println!("var clone = {}", varclone);
	println!("param move by default = {}", parammove);

	// Reference and Borrow

	// one &mut, or multiple &, no both
	let varref : &String = &varmove;
	// let varrefmut : &mut String = &mut varmove; // compilation error
	
	let paramref : &String = fun_ref(&varmove);
	let mut parammut : String = varmove.clone();

	println!("var ref = {}", varref);
	println!("param ref = {}", paramref);
	println!("param mut = {}", parammut);
	println!("param ref mut = {}", fun_refmut(&mut parammut));
	println!("param mut = {}", parammut);

	// Slices

	let mut strsrc = String::from("Hello");
	let mut arrsrc = [0, 1, 2, 3, 4];

	let strslice = &strsrc[0..3];
	let arrslice = &arrsrc[0..3];

	println!("string slice = {}", strslice);
	strsrc.clear();
	// println!("string slice = {}", strslice); // compilation error

	println!("array slice = {:?}", arrslice);
	arrsrc[0] = 10;
	// println!("array slice = {:?}", arrslice); // compilation error
}