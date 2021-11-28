use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
	let c = Arc::new(Mutex::new(0)); // Arc => converts ref to primitive => A thread-safe reference-counting pointer. 'Arc' stands for 'Atomically Reference Counted'.
								 // 0 => init value

	let mut list = vec![];

	for _ in 1..10 {
		let c = Arc::clone(&c);
		let h = thread::spawn(move || {
			let mut num = c.lock().unwrap();
			*num += 1;
			println!("{}", num);
		});
		// unblock auto => as it out of scope
		list.push(h);
	}
	for h in list {
		h.join().unwrap();
	}

	println!("res => {}", c.lock().unwrap());
}
