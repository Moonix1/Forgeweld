use forgeweld::{core::entry::forgeld_run, prelude::*};

struct Sandbox {}

impl ApplicationTrait for Sandbox {
	fn init(&mut self) -> Box<dyn ApplicationTrait> {
		Box::new(Sandbox {})
	}
}

fn main() {
	forgeld_run(Box::new(Sandbox {}));
}
