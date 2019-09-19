use log::*;
use stdweb::console;

static LOGGER: Logger = Logger;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        match record.level() {
            Level::Error => console!(error, format!("{}", record.args())),
            _ => console!(log, format!("{}", record.args())),
        }
    }

    fn flush(&self) {}
}

pub fn init() {
    set_logger(&LOGGER).unwrap();
    set_max_level(LevelFilter::Info);
}
