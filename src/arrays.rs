use std::mem;

// fixed length
pub fn run() {
	let nums: [i32; 5] = [1, 2, 3, 4, 5]; // length of 5

	let slice: &[i32] = &nums[0..2]; // [1,2]

	println!(
		"{:?}",
		(nums, nums[4], nums.len(), mem::size_of_val(&nums), slice)
	);
}
