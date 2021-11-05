use std::collections::HashMap;

pub fn run() {
	let mut map: HashMap<String, u32> = HashMap::new();
	map.insert(String::from("hello"), 1);
	map.insert(String::from("worlds"), 2);

	for (key, value) in &map {
		println!("{:?}", (key, value));
	}

	match map.get(&String::from("hello")) {
		Some(&n) => println!("{}", n),
		None => println!("none found"),
	}

	map.remove(&String::from("hello"));
}
