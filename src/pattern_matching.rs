pub fn run() {
	let x = 2;
	match x {
		1 => println!("one"),
		2 | 3 | 5 => println!("i won"),
		6..=10 => println!("i overflown"),
		_ => println!("unorthodox"),
	}
	let pair = (1, 2);
	match pair {
		(x, 2) => println!("2 => {}", x),
		(1, y) => println!("2 => {}", y),
		_ => println!("none"),
	}

	// guards
	match pair {
		(x, y) if x == y => println!("equals"),
		(x, y) if x > y => println!("greater"),
		_ => println!("none"),
	}

	let p = 10;
	match p {
		n @ 1..=10 => println!("n {}", n), // bind p's matched value to n => useful when dont have ownership of p
		_ => println!("none"),
	}

	let n6 = match p {
		n @ 1..=10 => n,
		_ => -1,
	};

	println!("{}", n6);
}
