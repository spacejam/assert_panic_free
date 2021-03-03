# assert_panic_free

A lightweight higher-order-function that doesn't compile
if a function you pass to it might panic. This probably
wont' work unless you're compiling your code with optimizations
enabled.

# Example

works when built with optimizations / release:

```no_build
assert_panic_free::assert_panic_free(|| 32);
```

doesn't work:

```compile_fail
assert_panic_free(|| panic!(":("));
```

Inspired by [this](https://github.com/dtolnay/no-panic) and [this](https://github.com/japaric/panic-never) but without requiring proc macros and corresponding long compilation times.
