use std::sync::mpsc;
use std::thread; // mpsc => multiple producer single consumer

fn play_with_channels() {
	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		tx.send(47).unwrap();
	});
	println!("{:?}", rx.recv().unwrap()); // recv => blocks the main thread until the message is received
}

pub fn run() {
	play_with_channels();
}
