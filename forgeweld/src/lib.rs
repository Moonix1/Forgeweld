pub mod core {
	pub mod defines;
	pub mod log;
	pub mod application;
	pub mod entry;
}

pub mod prelude {
	pub use crate::core::application::*;
}
