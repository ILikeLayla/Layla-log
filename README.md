# Layla-log

A simple logger library. This library provides a simple log writer and simple log level control. It can be used to write logs in a program. The logs can be written to the aimed dictionary. The log level can be set to different levels (Error, Warn, Debug, and Info).

## Usage

This can be initialized by using default setting, only the path of the aim dictionary is needed. 
```rust
use layla_log::*;

fn main() {
    default_init("path/to/log/directory/");
    clean_log(); // This will clean the log file.
    error!("error message.");
    warn!("warn message.");
    info!("info message.");
    debug!("debug message.");
    trace!("trace message.");
}
```

And this will be the output in the log file.
```log
{time} ERROR error message.
{time} WARN warn message.
{time} DEBUG debug message.
{time} INFO info message.
{time} TRACE trace message.
```
(Because the default log level is TRACE, so all the log will be recorded.)

And the time format is "yyyy-MM-dd HH:mm:ss.SSS". (The millisecond is included.)

It also provides a function for personal initializing, the following can be decided.
- The path of the aim dictionary.
- Maximum number of log in a single file. (0 as inf.)
- The restriction of the log level.
- Time zone.
- Show detailed time or not.
 
```rust
use layla_log::*;

fn main() {
    init("path/to/log/directory/", Some(200), Some(LogLevel::Debug) , 0, false);
    clean_log();
    error!("error message.");
    warn!("warn message.");
    info!("info message.");
    debug!("debug message.");
    trace!("trace message.");
}
```

And this will be the output.
```log
{time} ERROR error message.
{time} WARN warn message.
{time} INFO info message.
{time} DEBUG debug message.
```