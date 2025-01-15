# Layla-log

A simple logger library. This library provides a simple log writer and simple log level control. It can record logs to a target directory and also print them to terminal. The log can be set to different levels (Error, Warn, Debug, Info and Trace). And only the logs with significant level will be recorded to file or printed to terminal. Moreover, when the log file size exceeds a certain limit, it will automatically route to new files with indexing.

## Features list
- async (details see [async](./doc/async.md))

## Usage

Macros are provided to use the log writer easily, but before it here are several setting for the writer:

- dir_path
  - where the log file will be saved
  - the directory will be created if it does not exist
- single_length
  - the maximum length of a single log file (0 as unlimited)
- file_record_level
  - the minimum level of log that will be recorded to file
- terminal_print_level
  - the minimum level of log that will be printed to terminal
- time_zone
  - the time zone of the log file name and log message time
- time_detailed_display
  - whether to display detailed time in log message (whether time zone is included)
- print_out
  - whether to print log to terminal
- disabled
  - whether disable the logger or not


And this is an example:

```rust
use layla_log::*;

fn main() {
    init(Setting {
        dir_path: "/path/to/dir",
        single_length: 1219,
        file_record_level: LogLevel::Trace,
        terminal_print_level: LogLevel::Debug,
        time_detailed_display: true,
        time_zone: 0,
        print_out: true,
        disabled: false
    });
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}
```

and these are the output in the log file:

```log
{TIME} (+00:00) ERROR	This is an error message
{TIME} (+00:00) WARN	This is a warning message
{TIME} (+00:00) DEBUG	This is a debug message
{TIME} (+00:00) INFO	This is an info message
{TIME} (+00:00) TRACE	This is a trace message
```

and these are the output in the terminal:

```log
{TIME} (+00:00) ERROR  This is an error message
{TIME} (+00:00) WARN   This is a warning message
{TIME} (+00:00) DEBUG  This is a debug message
{TIME} (+00:00) INFO   This is an info message
```

Furthermore, all the setting have a default value:
- `"./logs/"` as the default dir_path
- `0` as the default log file single_length
- `LogLevel::Trace` as the default file_recode_level
- `LogLevel::Debug` as the default terminal_print_level for debug assertions, `LogLevel::Info` for default terminal_print_level for release assertions
- `0` as the default time_zone offset
- `false` as the default time_detailed_display
- `true` as the default print_out
- `false` as the default disabled

These default settings can be used by:
- Using default setting to initialize the logger
- No explicit initialization.

Here is an example using `default_init()`:

```rust
use layla_log::*;

fn main() {
    init(Setting::default());
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}
```

and these are the output in the log file:

```log
{TIME} (+00:00) ERROR	This is an error message
{TIME} (+00:00) WARN	This is a warning message
{TIME} (+00:00) DEBUG	This is a debug message
{TIME} (+00:00) INFO	This is an info message
{TIME} (+00:00) TRACE	This is a trace message
```

and these are the output in the terminal:

```log
{TIME} (+00:00) ERROR  This is an error message
{TIME} (+00:00) WARN   This is a warning message
{TIME} (+00:00) DEBUG  This is a debug message
```

Here is an example without any explicit initialization:

```rust
use layla_log::*;

fn main() {
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}
```

and the both output in the log file and the terminal are the same as using the first method.

In some cases, log is only used for debugging, and need to clean the log files each time the program runs, then `clean_log()` can be applied to clear the log file.

```rust
use layla_log::clean_log;

fn main() {
    clean_log();
}
```

And also, if some setting will be changed after while, then a `set` method can be used. (details see [init and set](./doc/init_and_set.md))

```rust
use layla_log::{init, set, Setting};

fn main() {
    init(Setting {
        // something
    });
    
    // something
    
    set(Setting {
        // something
    })
}
```

And if, the logger will be disabled and then enabled after some codes, then `disable_log` and `enable_log` method can be used.

```rust
use layla_log::*;

fn main() {
    disable_log();
    
    // logs will not be recorded or printed
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
    
    enable_log();
    
    // logs can be recorded or printed (with significant level)
    error!("This is an error message");
    warn!("This is a warning message");
    debug!("This is a debug message");
    info!("This is an info message");
    trace!("This is a trace message");
}
```

And if, you may want to announce the level by yourself instead of using corresponding macro, then you can use `log!` macro.

```rust
use layla_log::*;

fn main() {
    log!(LogLevel::Trace, "Hello, {}!", "world");
}

```

## Method list

- `init(setting: Setting)`
- `set(setting: Setting)`
- `disable_log()`
- `enable_log()`

## Maceo list

- `trace!`
- `info!`
- `debug!`
- `warn!`
- `error!`
- `log!`

## Cases

### Double Initialization
This happens when `init()` or `default_init()` is called more than once. In this case, logger won't be initialized again, but a warn log will be recorded (printed) with content `"Log writer has been initialized!"`.
