use std::{fmt::Display, ops::Index};

use super::MaybeMut;

pub type Position = (usize, usize);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl Direction {
	const DIRECTIONS: [Direction; 8] = [
		Direction::Up,
		Direction::Down,
		Direction::Left,
		Direction::Right,
		Direction::UpLeft,
		Direction::UpRight,
		Direction::DownLeft,
		Direction::DownRight,
	];
	
	pub fn each() -> impl Iterator<Item = Self> {
		Self::DIRECTIONS.into_iter()
	}
}

#[derive(Debug)]
pub struct Grid<T> {
	items: Vec<T>,
	width: usize,
}

impl<T> Grid<T> {
	pub fn empty() -> Self {
		Self::default()
	}
	
	pub fn is_empty(&self) -> bool {
		self.items.is_empty()
	}
	
	pub fn width(&self) -> usize {
		self.width
	}
	
	pub fn height(&self) -> usize {
		self.items.len() / self.width
	}
	
	pub fn rows(&self) -> impl Iterator<Item = &[T]> {
		(0..self.height())
			.map(|y| &self.items[y * self.width()..(y + 1) * self.width()])
	}
	
	pub fn positions(&self) -> impl Iterator<Item = (Position, &T)> {
		(0..self.height())
			.flat_map(move |y| (0..self.width())
				.map(move |x| ((x, y), &self[(x, y)]))
			)
	}
	
	pub fn step(&self, mut position: impl MaybeMut<Position>, direction: Direction) -> Option<&T> {
		if self.is_empty() {
			return None;
		}
		
		let (x, y) = position.as_mut();
		
		match direction {
			Direction::Left if *x > 0 => *x -= 1,
			Direction::Up if *y > 0 => *y -= 1,
			Direction::Right if *x < self.width() - 1 => *x += 1,
			Direction::Down if *y < self.height() - 1 => *y += 1,
			Direction::UpLeft if *x > 0 && *y > 0 => {*x -= 1; *y -= 1;},
			Direction::DownLeft if *x > 0 && *y < self.height() - 1 => {*x -= 1; *y += 1;},
			Direction::UpRight if *x < self.width() - 1 && *y > 0 => {*x += 1; *y -= 1;},
			Direction::DownRight if *x < self.width() - 1 && *y < self.height() - 1 => {*x += 1; *y += 1;},
			_ => return None,
		}
		
		Some(&self[(*x, *y)])
	}
}

impl<T> Index<Position> for Grid<T> {
	type Output = T;
	
	fn index(&self, (x, y): Position) -> &Self::Output {
		&self.items[x + y * self.width]
	}
}

impl<T, Row> FromIterator<Row> for Grid<T>
where
	Row: IntoIterator<Item = T>,
{
	fn from_iter<Rows: IntoIterator<Item = Row>>(iter: Rows) -> Self {
		let mut iter = iter.into_iter();
		let (column_hint, _) = iter.size_hint();
		
		let Some(first_row) = iter.next() else {
			return Self::empty();
		};
		
		let first_row = first_row.into_iter();
		let (row_hint, _) = iter.size_hint();
		
		let mut items = Vec::with_capacity(column_hint * row_hint);
		items.extend(first_row);
		let width = items.len();
		
		for row in iter {
			let prev_len = items.len();
			items.extend(row);
			let current_width = items.len() - prev_len;
			if current_width != width {
				panic!("Can't make Grid from rows of differing sizes");
			}
		}
		
		Self {
			items,
			width,
		}
	}
}

impl<T> Default for Grid<T> {
	fn default() -> Self {
		Self {
			items: Vec::default(),
			width: 0,
		}
	}
}

impl<T> Display for Grid<T>
where
	T: Display,
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		for row in self.rows() {
			for item in row {
				write!(f, "{item}")?;
			}
			writeln!(f)?;
		}
		
		Ok(())
	}
}
