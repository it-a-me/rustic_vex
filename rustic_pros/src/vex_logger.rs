use crate::libc::stdio;
use log::{max_level, LevelFilter, Metadata, Record};
use no_std_compat::prelude::v1::*;

pub fn init(max_level: LevelFilter) {
    log::set_logger(&crate::LOGGER)
        .map(|()| log::set_max_level(max_level))
        .unwrap()
}

pub struct VexLogger;

impl log::Log for VexLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let output = format!("{} - {}\n", record.level(), record.args());
            unsafe {
                stdio::printf(output.as_ptr().cast());
            }
        }
    }

    fn flush(&self) {}
}
