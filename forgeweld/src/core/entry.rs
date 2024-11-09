use crate::{core::log::*, prelude::ApplicationTrait};

pub fn forgeld_run(mut entry_struct: Box<dyn ApplicationTrait>) {
	Log::init();

	let mut app = entry_struct.init();
	app.run();
}