pub fn run() {
	let s = Some("s");
	match s {
		Some(i) => println!("{}", i),
		None => println!("oops"),
	}

	// via if let binding => not exhaustive => just covering one match case
	if let Some(i) = s {
		println!("ok => {}", i)
	} else {
	}

	// while let
	let mut n1 = Some(1);
	while let Some(i) = n1 {
		if i > 10 {
			println!("quit");
			n1 = None;
		} else {
			println!("{}", i);
			n1 = Some(i + 2);
		}
	}
}
