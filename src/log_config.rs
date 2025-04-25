#[derive(Clone)]
pub struct LogConfig {
    pub quiet: bool,
    pub stats: bool,
}

impl Default for LogConfig {
    fn default() -> Self {
        LogConfig {
            quiet: false,
            stats: false,
        }
    }
}
