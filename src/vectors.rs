// enums in vectors
#[derive(Debug)]
enum GenericVector {
	Float(f64),
	Int(f64),
	Text(String),
}

pub fn run() {
	let x: Vec<i32> = vec![1, 2, 3]; // resizable array => length not known at compile time
	let mut x2: Vec<i32> = Vec::new();
	// can only have one type of value inside
	x2.push(1);
	x2.push(2);
	x2.push(3);
	x2.pop();
	let len = x2.len();
	let cap = x2.capacity();
	println!("{:?}", (x2, len, cap));

	let my_vec = vec![
		GenericVector::Int(142.0),
		GenericVector::Float(12.0),
		GenericVector::Text(String::from("hello")),
	];
	println!("{:?}", my_vec);
}
