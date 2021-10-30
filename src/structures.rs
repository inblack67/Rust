use std::fmt;

// custom types

#[derive(Debug)] // to print using debug trait as rust by default would not know the debug trait of a custom type
struct Object {
	width: u32,
	height: u32,
}

// namespace of Object
impl Object {
	// // methods
	// fn show(&self) {
	// 	println!(
	// 		"area is {} of object with width {} and height {}",
	// 		self.area(),
	// 		self.width,
	// 		self.height
	// 	);
	// }

	// related functions
	//  bind functs to our Object type
	fn area(&self) -> u32 {
		// self will automatically refer to the Object it is tied to
		self.width * self.height
	}

	// new Object
	fn new(width: u32, height: u32) -> Object {
		Object { width, height }
	}
}

// seperate impl for methods
impl Object {
	// methods
	fn show(&self) {
		println!(
			"area is {} of object with width {} and height {}",
			self.area(),
			self.width,
			self.height
		);
	}
}

// to enable => println("{}", obj);
impl fmt::Display for Object {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "width => {}, height => {}", self.width, self.height)
	}
}

fn area(obj: &Object) -> u32 {
	obj.height * obj.width
}

pub fn run() {
	let obj = Object {
		width: 32,
		height: 40,
	};
	println!(
		"area is {} of object with width {} and height {}",
		area(&obj),
		obj.width,
		obj.height
	); // use ref => to avoid taking the Object away
	println!("impl area {}", obj.area());

	let obj2 = Object::new(12, 15);
	println!("{:?}", (obj2.width, obj2.height, obj2.area()));
	obj2.show();

	println!("{:?}", obj2);
	println!("{}", obj);
}
