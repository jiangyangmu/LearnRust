fn main() {
	let blkexpr = { let a = 2; let b = 3; a + b };
	let arrexpr = [1; 5];
	let rangeexpr = 0..10;
	let closureexpr = | i:i32 | -> i32 { i * 2 };
	let matchexpr = | score:i8 | -> char
	{
		match score {
			90..=100	=> 'A',
			80..=89		=> 'B',
			70..=79		=> 'C',
			60..=69		=> 'D',
			0..=59		=> 'E',
			_		=> '?',
		}
	};

	println!("blkexpr = {}", blkexpr);
	println!("arrexpr = {:?}", arrexpr);
	println!("rangeexpr = {}..{}", rangeexpr.start, rangeexpr.end);
	println!("closureexpr(10) = {}", closureexpr(10));
	println!("matchexpr(73) = {}", matchexpr(73));
}
