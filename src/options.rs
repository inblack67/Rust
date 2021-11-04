// rust don't have "nil" => billion dollar error

// Option is a part of standard library and it looks like =>
enum MyOption<T> {
	Some(T), // something
	None,    // nothing
}

fn division(x: f64, y: f64) -> Option<f64> {
	if y == 0.0 {
		None
	} else {
		Some(x / y)
	}
}

pub fn run() {
	let res = division(44.0, 22.0);
	println!("{:?}", res);
	match res {
		Some(ans) => println!("44.0/22.0 => {:10}", ans), // limit to 10 decimal places
		None => println!("invalid input"),
	}
}
