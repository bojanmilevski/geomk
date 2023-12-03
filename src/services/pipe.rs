use crate::services::filter::Filter;

pub struct Pipe<T> {
	filters: Vec<Box<dyn Filter<T>>>,
}

impl<T> Pipe<T> {
	pub fn new() -> Self {
		Self { filters: Vec::new() }
	}

	pub fn add_filter(&mut self, filter: Box<dyn Filter<T>>) {
		self.filters.push(filter);
	}

	pub fn run_filters(&self, mut input: T) -> T {
		for filter in &self.filters {
			input = filter.execute(input);
		}

		input
	}
}
