use env_logger::Builder;
use chrono::Local;
use log::LevelFilter;

use std::io::Write;

pub struct Log {}

impl Log {
	pub fn init() {
		Builder::new()
        .format(|buf, record| {
            let now = Local::now().format("[%H:%M:%S]").to_string();  // Timestamp format
            let level = record.level();
            let level_color = match level {
                log::Level::Error => "31",
                log::Level::Warn => "33",
                log::Level::Info => "32",
                log::Level::Debug => "34",
                log::Level::Trace => "37",
            };

            writeln!(buf, "\x1b[{}m{} [{}] {}: {}\x1b[0m",
				level_color, now, record.module_path().unwrap_or("unknown"), level, record.args())
        })
		.filter_level(LevelFilter::Trace)
        .init();
	}
}