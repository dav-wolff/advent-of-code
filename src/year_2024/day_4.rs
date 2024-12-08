use crate::{utils::{DiagonalDirection, Grid}, Solution};
use super::Day;

impl Solution for Day<4> {
	type Result = usize;
	
	const TEST_INPUT_ONE: &str = "
		MMMSXXMASM
		MSAMXMSMSA
		AMXSXMAAMM
		MSAMASMSMX
		XMASAMXAMM
		XXAMMXXAMA
		SMSMSASXSS
		SAXAMASAAA
		MAMMMXMMMM
		MXMXAXMASX
	";
	const TEST_RESULT_ONE: usize = 18;
	const TEST_INPUT_TWO: &str = Self::TEST_INPUT_ONE;
	const TEST_RESULT_TWO: usize = 9;
	
	fn part_one(input: &str) -> usize {
		let grid = parse_grid(input);
		
		grid.positions()
			.filter_map(|(position, &item)| (item == 'X').then_some(position))
			.map(|position| {
				DiagonalDirection::each()
					.filter(|&direction| {
						let mut position = position;
						grid.step(&mut position, direction) == Some(&'M')
							&& grid.step(&mut position, direction) == Some(&'A')
							&& grid.step(&mut position, direction) == Some(&'S')
					})
					.count()
			})
			.sum()
	}
	
	fn part_two(input: &str) -> usize {
		let grid = parse_grid(input);
		
		grid.positions()
			.filter_map(|(position, &item)| (item == 'A').then_some(position))
			.filter(|&position| {
				let up_left = grid.step(position, DiagonalDirection::UpLeft);
				let up_right = grid.step(position, DiagonalDirection::UpRight);
				let down_left = grid.step(position, DiagonalDirection::DownLeft);
				let down_right = grid.step(position, DiagonalDirection::DownRight);
				
				[up_left, up_right, down_left, down_right].into_iter()
					.all(|item| matches!(item, Some('M' | 'S')))
					&& up_left != down_right
					&& up_right != down_left
			})
			.count()
	}
}

fn parse_grid(input: &str) -> Grid<char> {
	input.lines()
		.map(|line| line.chars())
		.collect()
}
