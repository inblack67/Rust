// public function
pub fn greet() {
	println!("hello worlds");
	println!("number {}", 1);
	println!("number {} one {} two and {} three", 1, 2, 3);
	println!("number {1} one {0} two and {2} three", 1, 2, 3); // positional args => with indexes => o/p => 2,1,3

	// named args
	println!("hello my name is {name}", name = "M&M");

	// placeholder traits
	println!("Binary {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

	// placeholder for debug trait
	println!("{:?}", (true, 12, "ok")); // print anything => helps while debugging

	// math
	println!("{num} + {num} = {sum}", num = 10, sum = 10 + 10);
}
