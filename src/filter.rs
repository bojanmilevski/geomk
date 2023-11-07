pub trait Filter<T> {
	fn execute(&self, input: T) -> T;
}
