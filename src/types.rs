pub fn run() {
	println!("max of i32 => {}", std::i32::MAX);

	let is_active: bool = true;
	let is_greater: bool = 10 > 30;

	let my_char = 'a';
	let emoji = '\u{1F600}'; // unicode

	println!("{:?}", (is_active, is_greater, my_char, emoji));
}
