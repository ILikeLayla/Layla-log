# Init and set

`init()` and `set()` are both the public functions to provide some setting to the `Logger`.

However, `init()` should **ONLY** be called at most once in a single time and before any logs being recorded, but `set()` can be called anytime, anywhere, for any times, and the `Logger` will response to the `Setting` passed.

Actually, calling `init()` for multiple times won't lead to a panic, instead, it will record a warn log: "Log writer had been initialized!", and the `Logger` won't response to the `Setting` passed in.