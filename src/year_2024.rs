use crate::{Context, Day};

mod day_1;

pub fn visit_days(context: Context) {
	context.visit_day::<Day<1>>();
}
