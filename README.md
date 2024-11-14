# Layla-log

A simple logger library. This library provides a simple log writer and simple log level control. It can record logs to a target directory and also print to the terminal. The log can be set to different levels (Error, Warn, Debug, Info and Trace). And only the logs with high enough level will be recorded to file or printed to terminal. Moreover, when the log file size exceeds a certain limit, it will automatically route to new files with indexing.

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

And this is an example:

```rust
use layla_log::*;

fn main() {
   init("/path/to/dir", 1219, LogLevel::Trace, LogLevel::Debug, 0, true, true);
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
- `200` as the default log file single_length
- `LogLevel::Trace` as the default file_recode_level
- `LogLevel::Debug` as the default terminal_print_level
- `0` as the default time_zone offset
- `false` as the default time_detailed_display
- `true` as the default print_out

These default settings can be used by:
- Using `default_init()` to initialize the logger
- No explicit initialization.

Here is an example using `default_init()`:

```rust
use layla_log::*;

fn main() {
    init("/path/to/dir"); // dir_path is still needed
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

and the both output in the log file and the terminal are the same as using `default_init()`.

In some case, log is only used for debugging, and need to clean the log files each time the program runs, then `clean_log()` can be applied to clear the log file.

```rust
use layla_log::clean_log;

fn main() {
    clean_log();
}
```

## Cases
### Double Initialization
This happens when `init()` or `default_init()` is called more than once. In this case, logger won't be initialized again, but a warn log will be recorded (printed) with content `"Log writer has been initialized!"`.