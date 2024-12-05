pub trait TupleMap<U> {
	type T;
	type Tuple;
	
	fn map(self, f: impl FnMut(Self::T) -> U) -> Self::Tuple;
}

type Identity<T, const U: u8> = T;

macro_rules! tuple_map_impl {
	($($n: tt),+) => {
		impl<T, U> TupleMap<U> for ($(Identity<T, $n>),+,) {
			type T = T;
			type Tuple = ($(Identity<U, {$n}>),+,);
			
			fn map(self, mut f: impl FnMut(Self::T) -> U) -> Self::Tuple {
				($(f(self.$n)),+,)
			}
		}
	}
}

tuple_map_impl!(0);
tuple_map_impl!(0, 1);
tuple_map_impl!(0, 1, 2);
tuple_map_impl!(0, 1, 2, 3);
tuple_map_impl!(0, 1, 2, 3, 4);
tuple_map_impl!(0, 1, 2, 3, 4, 5);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14);
tuple_map_impl!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
