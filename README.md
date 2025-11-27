# Layla-log

A simple logger library. This library provides a simple log writer and simple log-level control. It can record logs to a target directory and also print them to the terminal. The log can be set to different levels (Error, Warn, Debug, Info and Trace). Only the logs with significant levels will be recorded to file or printed to the terminal. Moreover, when the log file size exceeds a certain limit, it will automatically route to new files with indexing.

## Features list

- async (details see [async](./doc/async.md))

## Usage

It can be used without any pre-setting.

```rust
use layla_log::*;

fn main() {
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
```

and these are the output in the log file and the things are printed to terminal:

```log
TIME ERROR [SCOPE @ PATH] This is an error message
TIME WARN  [SCOPE @ PATH] This is a warning message
TIME INFO  [SCOPE @ PATH] This is an info message
TIME DEBUG [SCOPE @ PATH] This is a debug message
TIME TRACE [SCOPE @ PATH] This is a trace message
```

And by using the `log_set!` macro, some things can be customized.

- dir_path
  - where the log file will be saved
  - the directory will be created if it does not exist
- single_length
  - the maximum length of a single log file (0 as unlimited)
- file_record_level
  - the minimum level of log that will be recorded to a file
- terminal_print_level
  - the minimum level of log that will be printed to the terminal
- time_zone
  - the time zone of the log file name and log message time
- time_detailed_display
  - whether to display detailed time in log message (whether time zone is included)
- file_time_format
  - it decides the format of the time record in the file
- print_out
  - whether to print the log to terminal
- display_scope
  - set whether display the scope in the log message
- display_file
  - set whether display the file path in the log message

This is an example:

```rust
use layla_log::*;

fn main() {
    log_set! {
      single_length: 1219
    };
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
```

And those values which are not given, the setting will inherit the previous setting. The `LogSetting` will use the default values while initializing. And these are the default values:

- `"./logs/"` as the default dir_path
- `0` as the default log file single_length
- `LogLevel::Trace` as the default file_recode_level
- `LogLevel::Debug` as the default terminal_print_level for debug assertions, `LogLevel::Info` for default terminal_print_level for release assertions
- `0` as the default time_zone
- `"%Y-%m-%d"` as the default file_time_format
- `false` as the default time_detailed_display
- `true` as the default print_out
- `true` as the default display_path
- `true` as the default display_scope

In some cases, log is only used for debugging, and need to clean the log files each time the program runs, then `clean_log()` can be applied to clear the log file.

```rust
use layla_log::clean_log;

fn main() {
    clean_log();
}
```

If the logger is disabled and then enabled after some codes, then the `disable_log` and `enable_log` methods can be used.

```rust
use layla_log::*;

fn main() {
    disable_log();
    
    // logs will not be recorded or printed
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
    
    enable_log();
    
    // logs can be recorded or printed (with significant level)
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}
```

If you want to announce the level yourself instead of using the corresponding macro, you can use the `log!` macro.

```rust
use layla_log::*;

fn main() {
    log!(LogLevel::Trace, "Hello, {}!", "world");
}

```

## Method list

- `clean_log()`
- `disable_log()`
- `enable_log()`

## Macro list

- `trace!`
- `info!`
- `debug!`
- `warn!`
- `error!`
- `log!`
- `set_log!`
