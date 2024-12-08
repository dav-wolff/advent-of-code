#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
	Up,
	Down,
	Left,
	Right,
}

impl Direction {
	pub fn rotate_left(self) -> Self {
		use Direction::*;
		
		match self {
			Up => Left,
			Down => Right,
			Left => Down,
			Right => Up,
		}
	}
	
	pub fn rotate_right(self) -> Self {
		use Direction::*;
		
		match self {
			Up => Right,
			Down => Left,
			Left => Up,
			Right => Down,
		}
	}
}

impl TryFrom<char> for Direction {
	type Error = ();
	
	fn try_from(value: char) -> Result<Self, Self::Error> {
		Ok(match value {
			'^' => Self::Up,
			'v' => Self::Down,
			'<' => Self::Left,
			'>' => Self::Right,
			_ => return Err(()),
		})
	}
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DiagonalDirection {
	Up,
	Down,
	Left,
	Right,
	UpLeft,
	UpRight,
	DownLeft,
	DownRight,
}

impl DiagonalDirection {
	const DIRECTIONS: [DiagonalDirection; 8] = [
		DiagonalDirection::Up,
		DiagonalDirection::Down,
		DiagonalDirection::Left,
		DiagonalDirection::Right,
		DiagonalDirection::UpLeft,
		DiagonalDirection::UpRight,
		DiagonalDirection::DownLeft,
		DiagonalDirection::DownRight,
	];
	
	pub fn each() -> impl Iterator<Item = Self> {
		Self::DIRECTIONS.into_iter()
	}
}

impl From<Direction> for DiagonalDirection {
	fn from(value: Direction) -> Self {
		match value {
			Direction::Up => Self::Up,
			Direction::Down => Self::Down,
			Direction::Left => Self::Left,
			Direction::Right => Self::Right,
		}
	}
}
