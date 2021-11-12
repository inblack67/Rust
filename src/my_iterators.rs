trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>; // mutable
}

pub fn run() {
	let v = vec![1, 2, 3];
	let mut x = v.iter(); // auto next
	println!("{:?}", (x.next())); // Some(1)
	v.iter().next(); // iterate in a controlled way
	lazy_evens1();
	lazy_even2();
}

fn is_even(n: i32) -> bool {
	n % 2 == 0
}

// functional
fn lazy_even2() {
	let top = 10000;
	let s: i32 = (0..)
		.map(|n| n * n)
		.take_while(|&n| n < top)
		.filter(|&n| is_even(n))
		.fold(0, |s, i| s + i);
	println!("functional s => {}", s);
}

fn lazy_evens1() {
	let top = 10000;
	let mut c = 0;
	for n in 0.. {
		let x = n * n;
		if x >= top {
			break;
		} else if is_even(x) {
			c += x;
		}
	} // going till infinity but wont => rust is lazy
	println!("c1 {}", c);
}
