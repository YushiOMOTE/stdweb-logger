use lazy_static::lazy_static;
use log::*;
use std::sync::{Arc, Mutex};
use stdweb::js;

static LOGGER: Logger = Logger;

lazy_static! {
    static ref CONFIG: Arc<Mutex<Config>> = Arc::new(Mutex::new(Config::new()));
}

struct Logger;

struct Config {
    filter: LevelFilter,
    format: Box<dyn Fn(&Record) -> String + Send + Sync + 'static>,
}

impl Config {
    fn new() -> Self {
        Self {
            filter: LevelFilter::Trace,
            format: Box::new(|r| format!("{}: {}", r.level(), r.args())),
        }
    }
}

pub struct Builder;

impl Builder {
    fn new() -> Self {
        Self
    }

    pub fn format(self, fmt: impl Fn(&Record) -> String + Send + Sync + 'static) -> Self {
        CONFIG.lock().unwrap().format = Box::new(fmt);
        self
    }

    pub fn filter(self, filter: LevelFilter) -> Self {
        CONFIG.lock().unwrap().filter = filter;
        self
    }

    pub fn detail(self) -> Self {
        CONFIG.lock().unwrap().format = Box::new(|r| {
            format!(
                "{}: {} ({}({}))",
                r.level(),
                r.args(),
                r.file().unwrap_or("<unknown>"),
                r.line().unwrap_or(0),
            )
        });
        self
    }

    pub fn build(self) {
        if set_logger(&LOGGER).is_ok() {
            set_max_level(CONFIG.lock().unwrap().filter);
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, m: &Metadata) -> bool {
        match CONFIG.lock().unwrap().filter.to_level() {
            Some(level) => m.level() <= level,
            None => false,
        }
    }

    fn log(&self, record: &Record) {
        let s = (CONFIG.lock().unwrap().format)(record);

        match record.level() {
            Level::Error => js! { console.error(@{s}); },
            Level::Warn => js! { console.warn(@{s}); },
            Level::Info => js! { console.info(@{s}); },
            Level::Debug => js! { console.debug(@{s}); },
            Level::Trace => js! { console.trace(@{s}); },
        };
    }

    fn flush(&self) {}
}

pub fn init() {
    builder().filter(LevelFilter::Trace).build()
}

pub fn builder() -> Builder {
    Builder::new()
}

pub fn init_with_level(level: Level) {
    builder().filter(level.to_level_filter()).build()
}
