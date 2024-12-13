use std::fmt::{Debug, Display};

pub trait Answer: Display + Debug + PartialEq {}

pub trait Solution {
	type Result: Answer;
	
	const TEST_INPUT_ONE: &str;
	const TEST_RESULT_ONE: Self::Result;
	const TEST_INPUT_TWO: &str;
	const TEST_RESULT_TWO: Self::Result;
	
	fn part_one(input: &str) -> impl SolutionSet<Self::Result>;
	fn part_two(input: &str) -> impl SolutionSet<Self::Result>;
}

macro_rules! answer_impl {
	($($t:ty),+) => {
	$(
		impl Answer for $t {}
	)+
	}
}

answer_impl!(i8, i16, i32, i64, isize, u8, u16, u32, u64, usize);

pub trait SolutionSet<A: Answer> {
	const SOLUTION_COUNT: usize;
	const IS_DEFERRED: bool;
	
	fn solve(&self, solution_index: usize) -> A;
	fn solve_all(&self) -> impl Iterator<Item = A> {
		(0..Self::SOLUTION_COUNT)
			.map(|i| self.solve(i))
	}
}

impl<T> SolutionSet<T> for T
where
	T: Answer + Copy,
{
	const SOLUTION_COUNT: usize = 1;
	const IS_DEFERRED: bool = false;
	
	fn solve(&self, solution_index: usize) -> T {
		if solution_index > 0 {
			panic!("index out of bounds");
		}
		
		*self
	}
}

macro_rules! replace {
	($_t:tt, $sub:tt) => {$sub}
}

macro_rules! solution_set_impl {
	($($f:ident),+; $($n:tt),+) => {
		impl <A, $($f),+> SolutionSet<A> for ($($f,)+)
		where
			A: Answer,
		$(
			$f: Fn() -> A,
		)+
		{
			const SOLUTION_COUNT: usize = {
				$(replace!($n, 1) +)+ 0
			};
			const IS_DEFERRED: bool = true;
			
			fn solve(&self, solution_index: usize) -> A {
				match solution_index {
				$(
					$n => self.$n(),
				)+
					_ => panic!("index out of bound"),
				}
			}
		}
	}
}

solution_set_impl!(F0; 0);
solution_set_impl!(F0, F1; 0, 1);
solution_set_impl!(F0, F1, F2; 0, 1, 2);
solution_set_impl!(F0, F1, F2, F3; 0, 1, 2, 3);
solution_set_impl!(F0, F1, F2, F3, F4; 0, 1, 2, 3, 4);
solution_set_impl!(F0, F1, F2, F3, F4, F5; 0, 1, 2, 3, 4, 5);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6; 0, 1, 2, 3, 4, 5, 6);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7; 0, 1, 2, 3, 4, 5, 6, 7);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8; 0, 1, 2, 3, 4, 5, 6, 7, 8);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
solution_set_impl!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15; 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
