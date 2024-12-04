pub trait MaybeMut<T> {
	fn as_mut(&mut self) -> &mut T;
}

impl<T> MaybeMut<T> for T {
	fn as_mut(&mut self) -> &mut T {
		self
	}
}

impl<T> MaybeMut<T> for &mut T {
	fn as_mut(&mut self) -> &mut T {
		self
	}
}
