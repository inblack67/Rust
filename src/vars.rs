// vars =>
// immutable by default, block scoped, hold primitive to data or ref to data

pub fn run() {
	let name = "Hello"; // immutable
	let mut age = "18"; // mutable
	println!("my name is {} and my age is {}", name, age);
	age = "19";
	println!("my name is {} and my age is {}", name, age);

	const ID: i32 = 1; // uppercase and explicit type
	println!("id => {}", ID);

	// multiple vars
	let (my_name, my_age) = ("ok", 18);
	println!("{}, {}", my_name, my_age);
}
