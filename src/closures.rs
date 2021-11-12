// anonymous functions which can be saved in a variable, pass around as args into another funcs
pub fn run() {
	let anonymous_function = |i| i + 1; // closure
	let x = anonymous_function(10);
	let y = || println!("I am a clousre with no value");
	y();
	println!("{:?}", (x));

	let mut c = 1;
	let mut inc = || {
		c += 1; // borrowing c from outer scope
		println!("c has a new value of {}", c);
	};

	inc();
	inc();
	inc();

	let p1 = || println!("i do nothing");
	run_me(p1);
	let p2 = |i| i + 10;
	println!("add3 res = {}", add3(p2));

	let closure_struct = ClosureStruct { f: p2 };
	let x12 = closure_struct.f;
	let x12_res = x12(11);
	println!("x12_res {}", x12_res);

	let created = create();
	created(); // some fun

	// closures in iterators
	let v = vec![1, 2, 3];
	println!("v {:?}", (v.iter().any(|&x| x != 2)));
}

fn run_me<F>(f: F)
where
	F: Fn(),
{
	f();
}

fn add3<F>(f: F) -> i32
where
	F: Fn(i32) -> i32, // Fn trait => func which takes i32 and returns i32
{
	f(3)
}

struct ClosureStruct<F: Fn(i32) -> i32> {
	f: F,
}
// closure as o/p params => rust only supports concrete non generic types and anonymous closure have unkown types so gotta make them concrete
fn create() -> Box<dyn Fn()> {
	Box::new(move || println!("some fun")) // all the value inside this closure must be ref by value rather than ref => so that they dont die when create func ends
}
