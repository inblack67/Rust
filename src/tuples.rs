// max => 12

pub fn run() {
	let person: (&str, &str, &str, i8) = ("Me", "me@me.com", "secret", 123);
	println!(
		"person => {} with email of {}, password of {} and number of {}",
		person.0, person.1, person.2, person.3
	);
}
