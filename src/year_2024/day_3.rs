use crate::Solution;
use super::Day;

use regex::Regex;

impl Solution for Day<3> {
	type Result = u64;
	
	const TEST_INPUT_ONE: &str = "
		xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
	";
	const TEST_RESULT_ONE: u64 = 161;
	const TEST_INPUT_TWO: &str = "
		xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
	";
	const TEST_RESULT_TWO: u64 = 48;
	
	fn part_one(input: &str) -> u64 {
		let re = Regex::new("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
		re.captures_iter(input)
			.map(|captures| {
				captures[1].parse::<u64>().unwrap() * captures[2].parse::<u64>().unwrap()
			})
			.sum()
	}
	
	fn part_two(mut input: &str) -> u64 {
		let re_mul = Regex::new("(don't\\(\\))|mul\\(([0-9]{1,3}),([0-9]{1,3})\\)").unwrap();
		let re_do = Regex::new("do\\(\\)").unwrap();
		let mut total = 0;
		
		while let Some(captures) = re_mul.captures(input) {
			let match_end = captures.get(0).unwrap().end();
			input = &input[match_end..];
			
			if captures.get(1).is_some() { // don't
				let Some(matched) = re_do.find(input) else {
					break;
				};
				
				input = &input[matched.end()..];
			} else {
				total += captures[2].parse::<u64>().unwrap() * captures[3].parse::<u64>().unwrap();
			}
		}
		
		total
	}
}
