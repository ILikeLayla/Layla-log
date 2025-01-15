# async

Some async functions are announced to deal with the File IO, and they are the implementation of `Logger`.

Here is the list:

- `init()`
- `set()`
- `clear_dir()`
- `write()`
- `record()`
- `info()`
- `debug()`
- `warn()`
- `error()`
- `trace()`
- `get_file()`
- `get_index()`

And those which call the function above also have an async attribute, here is the list:

- `init()` in [lib.rs](../src/lib.rs)
- `clean_log()`
- `set()` in [lib.rs](../src/lib.rs)
- `error!`
- `warn!`
- `info!`
- `debug!`
- `trace!`
- `log!`
- `enable_log()`
- `disable_log()`