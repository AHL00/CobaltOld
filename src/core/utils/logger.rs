use std::fs::{File, OpenOptions};
use std::io::Write;

pub struct LogLevelConfig {
    pub debug: bool,
    pub info: bool,
    pub warn: bool,
    pub error: bool,
}

pub struct Logger {
    log_level: LogLevelConfig,
    thread_tx: std::sync::mpsc::Sender<LogMessage>,
}

enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct LogOutputConfig {
    pub file: bool,
    pub overwrite_file: bool,
    pub path: String,
    pub console: bool,
}

// Passed to the logger thread
struct LogMessage {
    pub level: LogLevel,
    pub text: String,
}

impl Logger {
    pub fn new(log_level: LogLevelConfig, output: LogOutputConfig) -> Result<Logger, String> {
        let (tx, rx) = std::sync::mpsc::channel::<LogMessage>();

        if !output.file && !output.console {
            Err("No output specified".to_string())?;
        }

        let mut log_file: Option<File> = None;

        // create log file
        if output.file {
            let path: std::path::PathBuf = output.path.clone().into();
            // check if it a directory
            if path.is_dir() || path.ends_with("/") || path.ends_with("\\") {
                Err("Log file path is a directory".to_string())?;
            }

            // check if file directory exists
            if let Some(parent) = path.parent() {
                if !parent.exists() {
                    Err("Log file directory does not exist".to_string())?;
                }
            }

            let mut file_exists = false;
            // check if file exists
            if path.exists() {
                tx.send(LogMessage {
                    level: LogLevel::Info,
                    text: "Log file already exists".to_string(),
                })
                .unwrap();

                file_exists = true;
            }

            // create file
            if !file_exists {
                match File::create(&path) {
                    Ok(file) => {
                        tx.send(LogMessage {
                            level: LogLevel::Info,
                            text: format!("Created log file at {}", path.to_str().unwrap()),
                        })
                        .unwrap();
                        log_file = Some(file);
                    }
                    Err(e) => {
                        Err(format!("Failed to create log file: {}", e))?;
                    }
                }
            } else if output.overwrite_file {
                match File::create(&path) {
                    Ok(file) => {
                        tx.send(LogMessage {
                            level: LogLevel::Info,
                            text: format!("Overwriting log file at {}", path.to_str().unwrap()),
                        })
                        .unwrap();
                        log_file = Some(file);
                    }
                    Err(e) => {
                        Err(format!("Failed to create log file: {}", e))?;
                    }
                }
            } else {
                match OpenOptions::new().append(true).open(&path) {
                    Ok(file) => {
                        tx.send(LogMessage {
                            level: LogLevel::Info,
                            text: format!("Opened log file at {}", path.to_str().unwrap()),
                        })
                        .unwrap();
                        log_file = Some(file);
                    }
                    Err(e) => {
                        Err(format!("Failed to open log file: {}", e))?;
                    }
                }
            }
        }

        // start logger thread
        std::thread::spawn(move || {

            while let Ok(msg) = rx.recv() {
                // get time formatted as HH:MM:SS
                let time = chrono::Local::now().format("%H:%M:%S");

                match msg.level {
                    LogLevel::Debug => {
                        println!("\x1B[1;34m[DEBUG][{}] {}\x1B[0m", time, msg.text);
                        if let Some(log_file) = &mut log_file {
                            writeln!(log_file, "[DEBUG][{}] {}", time, msg.text).expect("Failed to write to log file");
                        }
                    }
                    LogLevel::Info => {
                        println!("\x1B[1;32m[INFO][{}] {}\x1B[0m", time, msg.text);
                        if let Some(log_file) = &mut log_file {
                            writeln!(log_file, "[INFO][{}] {}", time, msg.text).expect("Failed to write to log file");
                        }
                    }
                    LogLevel::Warn => {
                        println!("\x1B[1;33m[WARN][{}] {}\x1B[0m", time, msg.text);
                        if let Some(log_file) = &mut log_file {
                            writeln!(log_file, "[WARN][{}] {}", time, msg.text).expect("Failed to write to log file");
                        }
                    }
                    LogLevel::Error => {
                        println!("\x1B[1;31m[ERROR][{}] {}\x1B[0m", time, msg.text);
                        if let Some(log_file) = &mut log_file {
                            writeln!(log_file, "[ERROR][{}] {}", time, msg.text).expect("Failed to write to log file");
                        }
                    }
                }
            }
        });
        Ok(Logger {
            log_level,
            thread_tx: tx,
        })
    }

    pub fn debug(&self, message: &str) {
        if self.log_level.debug {
            self.thread_tx
                .send(LogMessage {
                    level: LogLevel::Debug,
                    text: message.to_string(),
                })
                .unwrap();
        }
    }

    pub fn info(&self, message: &str) {
        if self.log_level.info {
            self.thread_tx
                .send(LogMessage {
                    level: LogLevel::Info,
                    text: message.to_string(),
                })
                .unwrap();
        }
    }

    pub fn warn(&self, message: &str) {
        if self.log_level.warn {
            self.thread_tx
                .send(LogMessage {
                    level: LogLevel::Warn,
                    text: message.to_string(),
                })
                .unwrap();
        }
    }

    pub fn error(&self, message: &str) {
        if self.log_level.error {
            self.thread_tx
                .send(LogMessage {
                    level: LogLevel::Error,
                    text: message.to_string(),
                })
                .unwrap();
        }
    }
}

#[test]
fn test_logger() {
    let logger: Logger;

    match Logger::new(
        LogLevelConfig {
            debug: true,
            info: true,
            warn: true,
            error: true,
        },
        LogOutputConfig {
            file: true,
            overwrite_file: true,
            path: "./game.log".to_string(),
            console: true,
        },
    ) {
        Ok(l) => {
            logger = l;
        }
        Err(e) => {
            println!("Failed to create logger: {}", e);
            panic!();
        }
    }

    logger.debug("debug");
    logger.info("info");
    logger.warn("warn");
    logger.error("error");

    std::thread::sleep(std::time::Duration::from_secs(1));
}
