use crate::prelude::ApplicationTrait;

pub fn forgeld_run(mut entry_struct: Box<dyn ApplicationTrait>) {
	let mut app = entry_struct.init();
	app.run();
}