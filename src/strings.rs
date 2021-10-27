// primitive str => immutable fixed length
// String => Growable, heap allocated => use when you need to modify or own string data

pub fn run() {
	let mut greet = String::from("hello ");

	let len = greet.len();

	greet.push('w');
	greet.push_str("orlds");

	let capacity = greet.capacity();

	let is_empty = greet.is_empty();

	let contains = greet.contains("worlds");

	let new_greet = greet.replace("worlds", "world");

	for word in greet.split_whitespace() {
		println!("{}", word);
	}

	let string_with_capacity = String::with_capacity(10);

	assert_eq!(10, string_with_capacity.capacity()); // panics if fails else does nothing

	println!(
		"{:?}",
		(
			greet,
			len,
			capacity,
			is_empty,
			contains,
			new_greet,
			string_with_capacity
		)
	);
}
