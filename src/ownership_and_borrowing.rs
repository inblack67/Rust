fn take(v: Vec<i32>) {
	println!("{:?}", v);
}

fn copy(a: i32, b: i32) -> i32 {
	a + b
}

fn re(v: Vec<i32>) -> Vec<i32> {
	println!("{:?}", v);
	v
}

fn borrow1(v: &Vec<i32>) {
	println!("{:?}", ((*v)[0]));
}

// sugar
fn borrow2(v: &Vec<i32>) {
	println!("{:?}", (v[1])); // wont give memory address but will follow that ref t0 it's actual value
}

fn count_without_borrow(v: &Vec<i32>, val: i32) -> usize {
	v.into_iter().filter(|&&x| x == val).count()
}

pub fn run() {
	let message = String::from("hello");
	// let x = message; // moving the ref from message to x => after this line the actual ref to message disappears entirely from the scope => only one rf can own one piece of data at a time

	let _x = &message; // borrow message

	println!("{:?}", (message));

	// dynamic vector
	let mut v: Vec<i32> = Vec::new(); // heap
	for i in 1..10 {
		// 10 is exclusive
		v.push(i);
	}
	// moving
	take(v); // transfering ownership from run function to take and never return ownership back to run function

	let (a, b) = (10, 20); // stack => exists in both scope => run's and copy's
	copy(a, b); // copying not borrowing

	// borrowing

	let mut v2: Vec<i32> = Vec::new();
	for i in 1..10 {
		v2.push(i);
	}
	v2 = re(v2); // transferring back the ownership
	println!("{:?}", v2);
	borrow1(&v2);
	borrow2(&v2);
	println!("{:?}", v2);

	let v4 = vec![1, 2, 3, 3, 44, 4, 5, 5, 5];

	for &i in &v4 {
		let r = count_without_borrow(&v4, i);
		println!("{} is repeated {} times", i, r);
	}
}
