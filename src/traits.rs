use std::ops;

// shared behaviour => like interfaces in other langs
trait Shape {
	fn area(&self) -> u32;
}

struct Rectangle {
	length: u32,
	breadth: u32,
}

impl Shape for Rectangle {
	fn area(&self) -> u32 {
		self.breadth * self.length
	}
}

// #[derive(Debug, Clone)] // Clone => whenever new ref points => clone it => dont pass the ownership
#[derive(Debug, Clone, Copy)] // copy auto => no need to use clone fn
struct A(u32);

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct B(u32);

struct P;
struct Q;

// operator overloading
#[derive(Debug)]
struct PQ;
#[derive(Debug)]
struct QP;

impl ops::Add<P> for Q {
	type Output = PQ;
	fn add(self, _rhs: P) -> PQ {
		PQ
	}
}

impl ops::Add<Q> for P {
	type Output = QP;
	fn add(self, _rhs: Q) -> QP {
		QP
	}
}

struct Intro {
	name: String,
}

// called auto when var becomes out of scope
impl Drop for Intro {
	fn drop(&mut self) {
		println!("dropped from memory {:?}", self.name);
	}
}

struct Fib {
	c: u32,
	n: u32,
}

impl Iterator for Fib {
	type Item = u32;
	fn next(&mut self) -> Option<u32> {
		let n = self.c + self.n;
		self.c = self.n;
		self.n = n;
		Some(self.c)
	}
}

fn fib() -> Fib {
	Fib { c: 1, n: 1 }
}

pub fn run() {
	let rect = Rectangle {
		length: 12,
		breadth: 12,
	};
	let rect_area = rect.area();
	println!("{:?}", rect_area);
	let a = A(32);
	let b = a.clone();
	println!("{:?}", (a.0, b.0));
	println!("{:?}", P + Q);
	println!("{:?}", Q + P);

	let i1 = Intro {
		name: String::from("i1"),
	};
	{
		let i2 = Intro {
			name: String::from("i2"),
		};
		{
			let i3 = Intro {
				name: String::from("i3"),
			};
			println!("{:?} about to be dropped", i3.name);
		}
		println!("{:?} about to be dropped", i2.name);
	}
	// drop(i1); // explicit drop
	println!("{:?} about to be dropped", i1.name);

	for j in fib().take(10) {
		println!("{}", j)
	}

	for j in fib().skip(14).take(10) {
		println!("{}", j)
	}

	let mut f = fib();
	println!("{:?}", (f.next(), f.next(), f.next()));
}
