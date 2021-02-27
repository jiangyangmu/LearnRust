use std::fmt;

struct Point { x : i32, y : i32, }
struct Pair ( i32, i32 );
#[allow(dead_code)]
enum Color { WHITE, BLACK }

impl fmt::Debug for Point {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.debug_struct("Point")
		.field("x", &self.x)
		.field("y", &self.y)
		.finish()
	}
}
impl fmt::Display for Pair {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(fmt, "({}, {})", self.0, self.1)
	}
}

fn main() {
	let itype : i32 = -100;
	let utype : u32 = 100;
	let ctype : char = '符';
	let btype : bool = true;
	let tuptype : (char, u32) = ('键', 2);
	let arrtype : [&str; 7] = ["shoot"; 7];
	let sttype1 : Point = Point { x: -200, y: 200 };
	let sttype2 : Pair = Pair(100, 200);
	let etype1 : Color = Color::BLACK;

	println!("itype = {}", itype);	
	println!("utype = {}", utype);	
	println!("ctype = {}", ctype);
	println!("btype = {}", btype);
	println!("tuptype = {:?}\n\tfirst item: {}", tuptype, tuptype.0);
	println!("arrtype = {:?}\n\tfirst item: {}", arrtype, arrtype[0]);
	println!("sttype1 = {:?}", sttype1);
	println!("sttype2 = {}", sttype2);
	println!("etype1 = {:?}", match etype1 { Color::WHITE => "WHITE", _ => "not WHITE"});
}