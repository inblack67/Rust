// enum Direction {
// 	UP(u32, u32),
// 	// DOWN { x: u32, y: u32 }, // even structs allowed
// 	DOWN { x: u32, y: u32 }, // even structs allowed
// 	LEFT,
// 	RIGHT,
// }

struct Point {
	x: u32,
	y: u32,
}

enum Direction {
	UP(Point), // tuple of point struct
	DOWN(Point),
	LEFT(Point),
	RIGHT(Point),
}

#[derive(Debug)]
enum Keys {
	UpKey(String),
	DownKey(String),
	LeftKey(String),
	RightKey(String),
}

impl Direction {
	fn match_direction(&self) -> Keys {
		match *self {
			Direction::UP(_) => Keys::UpKey(String::from("Up key pressed")),
			Direction::DOWN(_) => Keys::UpKey(String::from("Down key pressed")),
			Direction::LEFT(_) => Keys::UpKey(String::from("Left key pressed")),
			Direction::RIGHT(_) => Keys::UpKey(String::from("Right key pressed")),
		}
	}
}

impl Keys {
	fn destruct(&self) -> &String {
		match *self {
			Keys::UpKey(ref s) => s,
			Keys::DownKey(ref s) => s,
			Keys::LeftKey(ref s) => s,
			Keys::RightKey(ref s) => s,
		}
	}
}

enum Shape {
	Rectangle { length: u32, breadth: u32 },
	Circle(f64),
	Square(u32),
}

// polymorphism
impl Shape {
	fn area(&self) -> f64 {
		match *self {
			Shape::Rectangle { length, breadth } => (length * breadth) as f64,
			Shape::Square(ref s) => (s * s) as f64,
			Shape::Circle(ref r) => 3.14 * (r * r),
		}
	}
}

pub fn run() {
	let u = Direction::UP(Point { x: 8, y: 10 });
	let v = u.match_direction();
	let x = v.destruct();
	println!("{:?}", (u.match_direction(), x));

	let v1 = 10;
	let p1 = &v1;
	let ref p2 = v1;
	if p1 == p2 {
		println!("p1 === p2");
	}
	let myCircle = Shape::Circle(3.0);
	let mySquare = Shape::Square(4);
	let myRectangle = Shape::Rectangle {
		length: 2,
		breadth: 4,
	};
	let circleArea = myCircle.area();
	let squareArea = mySquare.area();
	let rectArea = myRectangle.area();
	println!("{:?}", (circleArea, squareArea, rectArea));
}
