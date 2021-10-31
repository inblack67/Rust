pub fn run() {
	let mut n = 2;

	// if else bindings => types need to be the same
	n = if n < 5 { n } else { -1 };
	println!("{:?}", n);

	// loop {
	// 	println!("i am an infinite loop");
	// }

	let mut n2 = 0;
	loop {
		n2 = n2 + 1;
		println!("running loop {} times", n2);
		if n2 >= 10 {
			break;
		}
	}

	// labelled loops
	// 'a: loop {
	// 	println!("loop a");
	// 	'b: loop {
	// 		println!("loop b");
	// 		'c: loop {
	// 			println!("loop c");
	// 			break 'b; // break loop b
	// 		}
	// 	}
	// }

	let mut n3 = 10;
	while n3 < 12 {
		println!("{}", n3);
		n3 = n3 + 1;
	}

	// 1..10 => exclusive range => 1-9
	// 1..=10 => inclusive range => 1-10
	for i in 1..10 {
		println!("{} times", i);
	}
}
