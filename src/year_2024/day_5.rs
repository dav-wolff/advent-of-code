use crate::{utils::TupleMap, Solution};
use super::Day;

impl Solution for Day<5> {
	type Result = u64;
	
	const TEST_INPUT_ONE: &str = "
		47|53
		97|13
		97|61
		97|47
		75|29
		61|13
		75|53
		29|13
		97|29
		53|29
		61|53
		97|53
		61|29
		47|13
		75|47
		97|75
		47|61
		75|61
		47|29
		75|13
		53|13
		
		75,47,61,53,29
		97,61,53,29,13
		75,29,13
		75,97,47,61,53
		61,13,29
		97,13,75,29,47
	";
	const TEST_RESULT_ONE: u64 = 143;
	const TEST_INPUT_TWO: &str = Self::TEST_INPUT_ONE;
	const TEST_RESULT_TWO: u64 = 123;
	
	fn part_one(input: &str) -> u64 {
		let (rules, updates) = parse_input(input);
		
		updates
			.filter(|pages| is_sorted(&rules, pages))
			.map(middle_page)
			.sum()
	}
	
	fn part_two(input: &str) -> u64 {
		let (rules, updates) = parse_input(input);
		
		updates
			.filter(|pages| !is_sorted(&rules, pages))
			.map(|mut pages| {
				sort(&rules, &mut pages);
				pages
			})
			.map(middle_page)
			.sum()
	}
}

fn middle_page(pages: Vec<u8>) -> u64 {
	assert!(pages.len() % 2 == 1);
	pages[pages.len() / 2] as u64
}

type Rule = (u8, u8);

fn parse_input(input: &str) -> (Vec<Rule>, impl Iterator<Item = Vec<u8>> + use<'_>) {
	let (rules, updates) = input.split_once("\n\n").unwrap();
	let rules: Vec<Rule> = rules.lines()
		.map(|line| -> Rule {
			let (lower, upper) = line.split_once('|').unwrap();
			(lower.parse().unwrap(), upper.parse().unwrap())
		})
		.collect();
	
	let updates = updates.lines()
		.map(|line| -> Vec<u8> {
			line.split(',')
				.map(|num| num.parse().unwrap())
				.collect()
		});
	
	(rules, updates)
}

fn is_sorted(rules: &[Rule], pages: &[u8]) -> bool {
	rules.iter().all(|rule| {
		let (Some(lower), Some(upper)) =
			rule.map(|page| pages.iter().position(|&p| p == page))
		else {
			return true;
		};
		
		lower <= upper
	})
}

fn sort(rules: &[Rule], pages: &mut Vec<u8>) {
	let mut did_swap = true;
	
	while did_swap {
		did_swap = false;
		
		for rule in rules {
			let (Some(lower), Some(upper)) =
				rule.map(|page| pages.iter().position(|&p| p == page))
			else {
				continue;
			};
			
			if lower > upper {
				pages.swap(lower, upper);
				did_swap = true;
			}
		}
	}
}
