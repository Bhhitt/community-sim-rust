use std::collections::VecDeque;
use log::{Record, Level, Metadata, SetLoggerError, LevelFilter};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct EventLog {
    pub events: VecDeque<String>,
    pub capacity: usize,
}

impl EventLog {
    pub fn new(capacity: usize) -> Self {
        EventLog {
            events: VecDeque::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, event: String) {
        if self.events.len() == self.capacity {
            self.events.pop_front();
        }
        self.events.push_back(event);
    }

    pub fn clear(&mut self) {
        self.events.clear();
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.events.iter()
    }
}

// --- Custom logger for piping log::info! to EventLog ---
pub struct EventLogLogger {
    pub event_log: Arc<Mutex<EventLog>>,
    pub level: LevelFilter,
}

impl log::Log for EventLogLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Only pipe info and above (customize if needed)
            if record.level() <= Level::Info {
                let msg = format!("{}", record.args());
                if let Ok(mut log) = self.event_log.lock() {
                    log.push(msg);
                }
            }
        }
    }

    fn flush(&self) {}
}

impl EventLogLogger {
    pub fn init(event_log: Arc<Mutex<EventLog>>, level: LevelFilter) -> Result<(), SetLoggerError> {
        let logger = Box::leak(Box::new(EventLogLogger { event_log, level }));
        log::set_logger(logger).map(|()| log::set_max_level(level))
    }
}

// --- Fern output for piping log messages to EventLog ---
#[derive(Clone)]
pub struct EventLogWriter {
    event_log: Arc<Mutex<EventLog>>,
}

unsafe impl Send for EventLogWriter {}
unsafe impl Sync for EventLogWriter {}

impl EventLogWriter {
    pub fn new(event_log: Arc<Mutex<EventLog>>) -> Self {
        Self { event_log }
    }
}

impl std::io::Write for EventLogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Ok(mut log) = self.event_log.lock() {
            if let Ok(msg) = std::str::from_utf8(buf) {
                log.push(msg.trim_end().to_string());
            }
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
