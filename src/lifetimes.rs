// we cant gurantee that the borrower returned from pr will live long enough to return back to main

// lifetime specifiers => 'a => same lifetime a,b & the o/p str
fn pr<'a>(x: &'a str, y: &'a str) -> &'a str {
	if x.len() == y.len() {
		x
	} else {
		y
	}
}

struct A<'a, 'b> {
	x: &'a str,
	y: &'b str,
}

// lifetime inference
pub fn run() {
	// static lifetime
	let s: &'static str = "static life"; // will live until the entire duration of the program
}
