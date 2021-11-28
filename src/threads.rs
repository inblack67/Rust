use std::thread; // rust uses operating system level threads, unlike elixir, erlang and go => they use green level threads

pub fn run() {
	let v = vec![1, 2, 3];
	// rust makes no gurantees about running order of threads
	let handle = thread::spawn(move || {
		println!("{:?}", v); // move keyword transfers ownership from main thread to this thread
	});
	// println!("{:?}", v); // wont work as ownership has been transfered via move

	let res = handle.join(); // to join the thread processing => or else main will finish first
	println!("{:?}", (res));
}
