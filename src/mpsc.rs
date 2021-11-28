use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_TIMERS: usize = 24; // usize => architechture dependent => 32bit or 64bit

fn timer(d: usize, tx: mpsc::Sender<usize>) {
	thread::spawn(move || {
		println!("sleeping for {}", d);
		thread::sleep(Duration::from_secs(d as u64));
		tx.send(d).unwrap();
	});
}

pub fn run() {
	let (tx, rx) = mpsc::channel();
	for i in 0..NUM_TIMERS {
		timer(i, tx.clone()); // 24 poducers
	}

	// 1 consumer
	for v in rx.iter().take(NUM_TIMERS) {
		println!("received {}", v);
	}
}
