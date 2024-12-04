use crate::{Context, Day};

mod day_1;
mod day_2;
mod day_3;

pub fn visit_days(context: Context) {
	context.visit_day::<Day<1>>();
	context.visit_day::<Day<2>>();
	context.visit_day::<Day<3>>();
}
