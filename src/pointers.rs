// every primitive is allocated in stack
// if wrapped in Box => heap

pub fn run() {
	let b = Box::new(10);
	println!("{:?}", b); // 10
}
