use std::fs::File;

// can also use Option for results but Result enum gives the reason of error
enum Result<T, E> {
	Ok(T),
	Err(E),
}

pub fn run() {
	let f = File::open("test.txt");
	let res = match f {
		Ok(file) => file,
		Err(error) => {
			panic!("no such file found {:?}", error);
		}
	};
	println!("{:?}", res);
}
