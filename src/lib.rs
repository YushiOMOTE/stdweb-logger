use log::*;
use std::fmt::Write;
use stdweb::js;

struct Logger {
    filter: LevelFilter,
    format: Box<dyn Fn(&mut String, &Record) -> std::fmt::Result + Send + Sync + 'static>,
}

impl Logger {
    fn new() -> Self {
        Self {
            filter: LevelFilter::Trace,
            format: Box::new(|s, r| write!(s, "{}: {}", r.level(), r.args())),
        }
    }
}

///
/// Builder object to build a logger
///
pub struct Builder {
    logger: Logger,
}

impl Builder {
    fn new() -> Self {
        Self {
            logger: Logger::new(),
        }
    }

    ///
    /// Set a function which is called every time logger formats strings
    ///
    pub fn format(
        mut self,
        fmt: impl Fn(&mut String, &Record) -> std::fmt::Result + Send + Sync + 'static,
    ) -> Self {
        self.logger.format = Box::new(fmt);
        self
    }

    ///
    /// Set log level filter
    ///
    pub fn filter(mut self, filter: LevelFilter) -> Self {
        self.logger.filter = filter;
        self
    }

    ///
    /// Show more detail (line numbers, file names etc.) in log
    ///
    pub fn detail(mut self) -> Self {
        self.logger.format = Box::new(|s, r| {
            write!(
                s,
                "{}: {} ({}({}))",
                r.level(),
                r.args(),
                r.file().unwrap_or("<unknown>"),
                r.line().unwrap_or(0),
            )
        });
        self
    }

    ///
    /// Sets the logger
    ///
    pub fn build(self) {
        let level = self.logger.filter.clone();
        if set_boxed_logger(Box::new(self.logger)).is_ok() {
            set_max_level(level);
        }
    }
}

impl log::Log for Logger {
    fn enabled(&self, m: &Metadata) -> bool {
        match self.filter.to_level() {
            Some(level) => m.level() <= level,
            None => false,
        }
    }

    fn log(&self, record: &Record) {
        let mut s = String::new();

        if let Err(e) = (self.format)(&mut s, record) {
            js! { @(no_return) console.error(@{e.to_string()}); }
        } else {
            match record.level() {
                Level::Error => js! { @(no_return) console.error(@{s}); },
                Level::Warn => js! { @(no_return) console.warn(@{s}); },
                Level::Info => js! { @(no_return) console.info(@{s}); },
                Level::Debug => js! { @(no_return) console.debug(@{s}); },
                Level::Trace => js! { @(no_return) console.trace(@{s}); },
            }
        }
    }

    fn flush(&self) {}
}

///
/// Initialize logger with default settings
///
pub fn init() {
    builder().build()
}

///
/// Create a builder for a logger
///
pub fn builder() -> Builder {
    Builder::new()
}

///
/// Initialize logger with specified log level
///
pub fn init_with_level(level: Level) {
    builder().filter(level.to_level_filter()).build()
}
