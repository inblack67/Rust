use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn play_with_files() {
	let f = File::open("text.txt");
	let f = match f {
		Ok(file) => file,
		Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("text.txt") {
			Ok(fc) => fc,
			Err(e) => {
				panic!("{}", e);
			}
		},
		Err(e) => {
			panic!("{}", e);
		}
	};
}

fn play_with_files_2() {
	let f = File::open("text.txt").unwrap(); // unwraps to data
	let f = File::open("text.txt").expect("Could not open file"); // unwraps to data or panics with the provided error
}

fn exit(x: Option<i32>) {
	match x {
		Some(0) => panic!("zero panic"), // compiler walks back up the stack => clean up the memory and then the panic happens
		Some(x) => println!("{}", x),
		None => println!("none match"),
	}
}

// propogating error
fn read_file() -> Result<String, io::Error> {
	let mut s = String::new();
	// ? -> can only be used in functions which return Result
	File::open("text.txt")?.read_to_string(&mut s)?; // if error => it will propogate to the caller of the func
	Ok(s)
}

pub fn run() {
	println!("{:?}", read_file());
	// play_with_files();

	// exit(Some(1));
	// exit(None);
	// exit(Some(0));
}
