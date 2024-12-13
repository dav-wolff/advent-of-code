#![forbid(unsafe_code)]
#![deny(non_snake_case)]
#![allow(refining_impl_trait)]

use std::{fs, io, iter, path::PathBuf, time::{Duration, Instant}};
use aoc_client::AocClient;
use clap::Parser;
use either::Either;
use solution::{Answer, Solution, SolutionSet};
use unindent::unindent;

mod solution;
mod utils;
mod year_2024;

#[derive(Parser, Debug)]
struct AocArgs {
	#[arg(long)]
	skip_tests: bool,
	#[arg(long)]
	skip_solve: bool,
	#[arg(long)]
	time: bool,
	#[arg(long)]
	day: Option<u8>,
}

fn main() {
	let args = AocArgs::parse();
	
	let mut context = Context {
		day: args.day,
		test: !args.skip_tests,
		solve: !args.skip_solve,
		time: args.time,
		was_visited: false,
	};
	
	year_2024::visit_days(&mut context);
	
	match (context.day, context.was_visited) {
		(Some(day), false) => {
			eprintln!("No solution available for day {day}");
		},
		_ => (),
	}
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
	time: bool,
	was_visited: bool,
}

impl Context {
	fn visit_day<S: Solution + DayNumber>(&mut self) {
		if let Some(day) = self.day {
			if day != S::DAY {
				return;
			}
		}
		
		self.was_visited = true;
		
		if self.test {
			test_part_one::<S>();
			test_part_two::<S>();
		}
		
		if self.solve || self.time {
			let print = |part, answer, time: Duration| {
				if self.time {
					println!("Day {} part {part} took {}ms", S::DAY, time.as_millis());
				}
				if self.solve {
					println!("Day {} part {part}: {answer}", S::DAY);
				}
			};
			
			let input = get_input(S::DAY);
			
			for (answer, time) in time_part(|| S::part_one(&input)) {
				print('1', answer, time);
			}
			for (answer, time) in time_part(|| S::part_two(&input)) {
				print('2', answer, time);
			}
		}
	}
}

fn time_part<F, S, A>(part: F) -> impl Iterator<Item = (A, Duration)>
where
	F: Fn() -> S,
	S: SolutionSet<A>,
	A: Answer,
{
	if !S::IS_DEFERRED {
		let start = Instant::now();
		let answer = part();
		let time = Instant::now() - start;
		return Either::Left(iter::once((answer.solve(0), time)));
	}
	
	let solution_set = part();
	let iter = (0..S::SOLUTION_COUNT)
		.map(move |i| {
			let start = Instant::now();
			let answer = solution_set.solve(i);
			let time = Instant::now() - start;
			(answer, time)
		});
	
	Either::Right(iter)
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
	let input = unindent(S::TEST_INPUT_ONE);
	let solution_set = S::part_one(&input);
	check_part(solution_set, S::TEST_RESULT_ONE);
}

fn test_part_two<S: Solution>() {
	let input = unindent(S::TEST_INPUT_TWO);
	let solution_set = S::part_two(&input);
	check_part(solution_set, S::TEST_RESULT_TWO);
}

fn check_part<A: Answer>(solution_set: impl SolutionSet<A>, expected: A) {
	for answer in solution_set.solve_all() {
		assert_eq!(answer, expected);
	}
}
