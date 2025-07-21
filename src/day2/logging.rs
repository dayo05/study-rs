use std::fmt::Display;

pub trait Logger {
    /// Log a message at the given verbosity level.
    fn log(&self, verbosity: u8, message: Box<dyn Display>);
}

pub struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: Box<dyn Display>) {
        eprintln!("verbosity={verbosity}: {:}", message.to_string());
    }
}

pub struct VerbosityFilter {
    max_verbosity: u8,
    inner: Box<dyn Logger>
}

impl VerbosityFilter {
    pub(crate) fn new(max_verbosity: u8, logger: Box<dyn Logger>) -> Self {
        VerbosityFilter { max_verbosity, inner: logger }
    }
}

impl Logger for VerbosityFilter {
    fn log(&self, verbosity: u8, message: Box<dyn Display>) {
        if verbosity <= self.max_verbosity {
            self.inner.log(verbosity, message);
        }
    }
}

pub struct StdoutLogger;

impl Logger for StdoutLogger {
    fn log(&self, verbosity: u8, message: Box<dyn Display>) {
        println!("verbosity={verbosity}: {message}");
    }
}
