use log::{warn};

pub trait ApplicationTrait {
	fn init(&mut self) -> Box<dyn ApplicationTrait>;

	fn run(&mut self) {
		warn!("Hello!");

		loop {}
	}
}