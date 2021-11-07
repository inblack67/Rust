use std::fmt;

// generic printing
fn p<T: fmt::Debug>(x: T) {
	println!("{:?}", x);
}

struct Square<T> {
	x: T,
}

struct A<T> {
	x: T,
}

impl<T> A<T> {
	fn item(&self) -> &T {
		&self.x
	}
}

struct M<U, V> {
	a: U,
	b: V,
}

pub fn run() {
	let s = Square { x: 1 };
	let s = Square { x: "hello" };
	let s = Square { x: true };
}
