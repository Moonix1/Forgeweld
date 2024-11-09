pub trait ApplicationTrait {
	fn init(&mut self) -> Box<dyn ApplicationTrait>;

	fn run(&mut self) {
		loop {}
	}
}