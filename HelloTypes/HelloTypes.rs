use std::fmt;

struct Point {
	x : i32,
	y : i32,
}

impl fmt::Display for Point {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "({}, {})", self.x, self.y)
	}
}
impl fmt::Debug for Point {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.debug_struct("Point")
		.field("x", &self.x)
		.field("y", &self.y)
		.finish()
	}
}

fn main() {
	let itype : i32 = -100;
	let utype : u32 = 100;
	let ctype : char = '符';
	let btype : bool = true;
	let tuptype : (char, u32) = ('键', 2);
	let arrtype : [&str; 7] = ["shoot"; 7];
	let pttype : Point = Point { x: -200, y: 200 };

	println!("itype = {}", itype);	
	println!("utype = {}", utype);	
	println!("ctype = {}", ctype);
	println!("btype = {}", btype);
	println!("tuptype = {:?}\n\tfirst item: {}", tuptype, tuptype.0);
	println!("arrtype = {:?}\n\tfirst item: {}", arrtype, arrtype[0]);
	println!("pttype = {} {:?}", pttype, pttype);
}