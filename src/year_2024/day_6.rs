use std::{cell::Cell, collections::HashSet, fmt::Display};

use crate::{solution::SolutionSet, utils::{Direction, Grid}, Solution};
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
			
			let obstable_positions: Vec<_> = map.positions()
				.filter(|&(position, _)| position != guard.position)
				.filter(|&(_, &field)| field == Field::Visited)
				.map(|(position, _)| position)
				.collect();
			
			obstable_positions.into_iter()
				.filter(|&position| {
					map[position] = Field::Obstacle;
					let does_loop = does_map_loop(&map, guard);
					map[position] = Field::Visited;
					does_loop
				})
				.count()
		},
		|| {
			let (map, guard) = parse_input(input);
			
			part_two_fast(map, guard)
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Guard {
	position: (usize, usize),
	direction: Direction,
}

fn parse_input(input: &str) -> (Grid<Field>, Guard) {
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
						guard.set(Some(Guard {
							position: (x, y),
							direction: char.try_into().unwrap(),
						}));
						Field::Visited
					},
					_ => panic!("unexpected input"),
				}
			})
		)
		.collect();
	
	(grid, guard.get().unwrap())
}

fn walk_map(map: &mut Grid<Field>, mut guard: Guard) {
	while take_step(map, &mut guard) {
		map[guard.position] = Field::Visited;
	}
}

fn take_step(map: &Grid<Field>, guard: &mut Guard) -> bool {
	while let Some(&field) = map.step(guard.position, guard.direction) {
		if field == Field::Obstacle {
			guard.direction = guard.direction.rotate_right();
		} else {
			map.step(&mut guard.position, guard.direction).unwrap();
			return true;
		}
	}
	
	false
}

fn does_map_loop(map: &Grid<Field>, mut guard: Guard) -> bool {
	let mut visited = HashSet::new();
	
	while let Some(&field) = map.step(guard.position, guard.direction) {
		if field == Field::Obstacle {
			guard.direction = guard.direction.rotate_right();
		} else {
			map.step(&mut guard.position, guard.direction).unwrap();
		}
		
		if visited.contains(&(guard.position, guard.direction)) {
			return true;
		}
		
		visited.insert((guard.position, guard.direction));
	}
	
	false
}

fn part_two_fast(mut map: Grid<Field>, mut guard: Guard) -> usize {
	let mut loop_count = 0;
	let mut visited = HashSet::new();
	visited.insert(guard);
	let mut obstacle = guard;
	
	while take_step(&map, &mut obstacle) {
		visited.insert(obstacle);
		
		if map[obstacle.position] == Field::Visited {
			guard = obstacle;
			continue;
		}
		
		map[obstacle.position] = Field::Obstacle;
		// let mut newly_visited = HashSet::new();
		let mut visited = visited.clone();
		
		while take_step(&map, &mut guard) {
			// if visited.contains(&guard) || newly_visited.contains(&guard) {
			if visited.contains(&guard) {
				loop_count += 1;
				break;
			}
			
			// newly_visited.insert(guard);
			visited.insert(guard);

		}
		
		map[obstacle.position] = Field::Visited;
		
		guard = obstacle;
	}
	
	loop_count
}
