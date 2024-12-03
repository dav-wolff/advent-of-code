use crate::Solution;
use super::Day;

impl Solution for Day<1> {
	type Result = u64;
	
	const TEST_INPUT_ONE: &str = "
		3   4
		4   3
		2   5
		1   3
		3   9
		3   3
	";
	const TEST_RESULT_ONE: u64 = 11;
	const TEST_INPUT_TWO: &str = Self::TEST_INPUT_ONE;
	const TEST_RESULT_TWO: u64 = 31;
	
	fn part_one(input: &str) -> u64 {
		let (mut left_list, mut right_list) = parse_lists(input);
		
		left_list.sort_unstable();
		right_list.sort_unstable();
		
		left_list.into_iter().zip(right_list)
			.map(|(left, right)| left.abs_diff(right))
			.sum()
	}
	
	fn part_two(input: &str) -> u64 {
		let (left_list, right_list) = parse_lists(input);
		
		left_list.into_iter()
			.map(|left| {
				let count = right_list.iter()
					.filter(|&&right| left == right)
					.count();
				left as u64 * count as u64
			})
			.sum()
	}
}

fn parse_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
	input.lines()
		.map(|line| {
			let mut iter = line.split_whitespace()
				.map(|num| -> i64 {num.parse().unwrap()});
			(iter.next().unwrap(), iter.next().unwrap())
		})
		.unzip()
}
