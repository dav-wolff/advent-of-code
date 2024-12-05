use crate::{Context, Day};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

pub fn visit_days(context: Context) {
	context.visit_day::<Day<1>>();
	context.visit_day::<Day<2>>();
	context.visit_day::<Day<3>>();
	context.visit_day::<Day<4>>();
	context.visit_day::<Day<5>>();
}
