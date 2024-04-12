# Layla-log

A simple logger library. This library provides a simple log writer and simple log level control. It can be used to write logs in a program. The logs can be written to the aimed dictionary. The log level can be set to different levels (Error, Warn, Debug, and Info).

## Usage

This can be initialized by using default setting, only the path of the aim dictionary is needed. 
```rust
use layla_log::{Writer, LogLevel};

fn main() {
    let mut writer = Writer::default("AIMED_DICTIONARY");
    writer.record(LogLevel::Error, "This is an error log.");
    writer.record(LogLevel::Warn, "This is a warning log.");
    writer.record(LogLevel::Debug, "This is a debug log.");
    writer.record(LogLevel::Info, "This is an info log.");
}
```

It also provides a function for personal initializing, the following can be decided.
- The path of the aim dictionary.
- The restriction of the log level.
- Show detailed time or not.
- Maximum number of log in a single file. (0 as inf.)
- Prefix of the log file.

Sometimes, a large amount of logs would be written. Then, the function `record` is not recommended. Instead, the combination of functions  `push` and `write_all` should be used.
 
```rust
use layla_log::{Writer, LogLevel};

fn main() {
    let mut writer = Writer::default("AIMED_DICTIONARY");
    for _ in 0..1_000_000 {
        writer.push(LogLevel::Info, "This is an info log.");
    }
    writer.write_all();
}
```