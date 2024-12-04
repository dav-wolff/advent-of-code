use crate::Solution;
use super::Day;

impl Solution for Day<2> {
	type Result = u64;
	
	const TEST_INPUT_ONE: &str = "
		7 6 4 2 1
		1 2 7 8 9
		9 7 6 2 1
		1 3 2 4 5
		8 6 4 4 1
		1 3 6 7 9
	";
	const TEST_RESULT_ONE: u64 = 2;
	const TEST_INPUT_TWO: &str = Self::TEST_INPUT_ONE;
	const TEST_RESULT_TWO: u64 = 4;
	
	fn part_one(input: &str) -> u64 {
		input.lines()
			.filter(|line| {
				let iter = line.split_whitespace()
					.map(|num| -> i64 {num.parse().unwrap()});
				let mut iter = iter.clone().zip(iter.skip(1))
					.map(|(left, right)| right - left);
				iter.clone().all(|diff| (1..=3).contains(&diff))
					|| iter.all(|diff| (-3..=-1).contains(&diff))
			})
			.count() as u64
	}
	
	fn part_two(input: &str) -> u64 {
		input.lines()
			.filter(|line| is_report_safe(line))
			.count() as u64
	}
}

fn is_report_safe(report: &str) -> bool {
	let iter = report.split_whitespace()
		.map(|num| -> i64 {num.parse().unwrap()});
	
	let count = iter.clone().count();
	
	let is_safe = |skipped_index| {
		let iter = iter.clone().enumerate()
			.filter(|(i, _)| *i != skipped_index)
			.map(|(_, num)| num);
		
		let mut iter = iter.clone().zip(iter.skip(1))
			.map(|(left, right)| right - left);
		
		iter.clone().all(|diff| (1..=3).contains(&diff))
			|| iter.all(|diff| (-3..=-1).contains(&diff))
	};
	
	(0..count).any(is_safe)
}
