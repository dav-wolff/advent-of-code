#![forbid(unsafe_code)]
#![deny(non_snake_case)]

use std::{fmt::{Debug, Display}, fs, io, path::PathBuf};
use aoc_client::AocClient;
use unindent::unindent;

mod utils;
mod year_2024;

fn main() {
	let context = Context {
		day: None,
		test: true,
		solve: true,
	};
	
	year_2024::visit_days(context);
}

trait Solution {
	type Result: Display + Debug + PartialEq;
	
	const TEST_INPUT_ONE: &str;
	const TEST_RESULT_ONE: Self::Result;
	const TEST_INPUT_TWO: &str;
	const TEST_RESULT_TWO: Self::Result;
	
	fn part_one(input: &str) -> Self::Result;
	fn part_two(input: &str) -> Self::Result;
}

struct Day<const D: u8>;

trait DayNumber {
	const DAY: u8;
}

impl<const N: u8> DayNumber for Day<N> {
	const DAY: u8 = N;
}

struct Context {
	day: Option<u8>,
	test: bool,
	solve: bool,
}

impl Context {
	fn visit_day<S: Solution + DayNumber>(&self) {
		if let Some(day) = self.day {
			if day != S::DAY {
				return;
			}
		}
		
		if self.test {
			test_part_one::<S>();
			test_part_two::<S>();
		}
		
		if self.solve {
			let input = get_input(S::DAY);
			let result = S::part_one(&input);
			println!("Day {} part 1: {result}", S::DAY);
			let result = S::part_two(&input);
			println!("Day {} part 2: {result}", S::DAY);
		}
	}
}

fn get_input(day: u8) -> String {
	let crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
	let inputs_dir = crate_dir.join("inputs");
	
	if !fs::exists(&inputs_dir).unwrap() {
		fs::create_dir(&inputs_dir).unwrap();
	}
	
	let file_location = inputs_dir.join(day.to_string());
	
	match fs::read_to_string(&file_location) {
		Ok(input) => return input,
		Err(err) if err.kind() != io::ErrorKind::NotFound => panic!("{err}"),
		Err(_) => (),
	}
	
	let aoc_client = AocClient::builder()
		.session_cookie_from_file(crate_dir.join("AOC_SESSION")).unwrap()
		.year(2024).unwrap()
		.day(day as u32).unwrap()
		.build().unwrap();
	
	let input = aoc_client.get_input().unwrap();
	fs::write(file_location, &input).unwrap();
	input
}

fn test_part_one<S: Solution>() {
	let result = S::part_one(&unindent(S::TEST_INPUT_ONE));
	assert_eq!(result, S::TEST_RESULT_ONE);
}

fn test_part_two<S: Solution>() {
	let result = S::part_two(&unindent(S::TEST_INPUT_TWO));
	assert_eq!(result, S::TEST_RESULT_TWO);
}
