use std::{cell::Cell, collections::HashSet, fmt::Display};

use crate::{solution::SolutionSet, utils::{Direction, Grid, Position}, Solution};
use super::Day;

impl Solution for Day<6> {
	type Result = usize;
	
	const TEST_INPUT_ONE: &str = "
		....#.....
		.........#
		..........
		..#.......
		.......#..
		..........
		.#..^.....
		........#.
		#.........
		......#...
	";
	const TEST_RESULT_ONE: usize = 41;
	const TEST_INPUT_TWO: &str = Self::TEST_INPUT_ONE;
	const TEST_RESULT_TWO: usize = 6;
	
	fn part_one(input: &str) -> usize {
		let (mut map, guard) = parse_input(input);
		walk_map(&mut map, guard);
		
		map.items()
			.filter(|&&field| field == Field::Visited)
			.count()
	}
	
	fn part_two(input: &str) -> impl SolutionSet<usize> {
		(|| {
			let (mut map, guard) = parse_input(input);
			walk_map(&mut map, guard);
			
			let positions: Vec<_> = map.positions()
				.filter(|&(position, _)| position != guard.0)
				.filter(|&(_, &field)| field == Field::Visited)
				.map(|(position, _)| position)
				.collect();
			
			positions.into_iter()
				.filter(|&position| {
					map[position] = Field::Obstacle;
					let does_loop = does_map_loop(&map, guard);
					map[position] = Field::Visited;
					does_loop
				})
				.count()
		},
		|| {
			let (mut map, guard) = parse_input(input);
			walk_map(&mut map, guard);
			
			let positions: Vec<_> = map.positions()
				.filter(|&(position, _)| position != guard.0)
				.filter(|&(_, &field)| field == Field::Visited)
				.map(|(position, _)| position)
				.collect();
			
			positions.into_iter()
				.filter(|&position| {
					map[position] = Field::Obstacle;
					let does_loop = does_map_loop(&map, guard);
					map[position] = Field::Visited;
					does_loop
				})
				.count()
		})
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Field {
	Obstacle,
	Unvisited,
	Visited,
}

impl Display for Field {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let c = match self {
			Field::Obstacle => '#',
			Field::Unvisited => '.',
			Field::Visited => 'X',
		};
		write!(f, "{c}")
	}
}

fn parse_input(input: &str) -> (Grid<Field>, (Position, Direction)) {
	let guard: &Cell<Option<_>> = &Cell::new(None);
	
	let grid: Grid<_> = input.lines()
		.enumerate()
		.map(|(y, line)| line.chars()
			.enumerate()
			.map(move |(x, char)| {
				match char {
					'#' => Field::Obstacle,
					'.' => Field::Unvisited,
					'^' | '<' | '>' | 'v' => {
						assert!(guard.get().is_none());
						guard.set(Some(((x, y), char.try_into().unwrap())));
						Field::Visited
					},
					_ => panic!("unexpected input"),
				}
			})
		)
		.collect();
	
	(grid, guard.get().unwrap())
}

fn walk_map(map: &mut Grid<Field>, (mut guard_position, mut guard_direction): (Position, Direction)) {
	while let Some(&field) = map.step(guard_position, guard_direction) {
		if field == Field::Obstacle {
			guard_direction = guard_direction.rotate_right();
		} else {
			let field = map.step_mut(&mut guard_position, guard_direction).unwrap();
			*field = Field::Visited;
		}
	}
}

fn does_map_loop(map: &Grid<Field>, (mut guard_position, mut guard_direction): (Position, Direction)) -> bool {
	let mut visited = HashSet::new();
	
	while let Some(&field) = map.step(guard_position, guard_direction) {
		if field == Field::Obstacle {
			guard_direction = guard_direction.rotate_right();
		} else {
			map.step(&mut guard_position, guard_direction).unwrap();
		}
		
		if visited.contains(&(guard_position, guard_direction)) {
			return true;
		}
		
		visited.insert((guard_position, guard_direction));
	}
	
	false
}
