# simavr-ffi &emsp; [![crates-badge]][crates-link]

[crates-badge]: https://img.shields.io/crates/v/simavr-ffi.svg
[crates-link]: https://crates.io/crates/simavr-ffi

Low-level unsafe bindings to [simavr](https://github.com/buserror/simavr):

```toml
[dependencies]
simavr-ffi = "1.1.0"
```

Supported platforms: Linux & Mac (pull requests for Windows etc. welcome!).

If you're looking for something high-level and safe, see [AvrTester](https://github.com/Patryk27/avr-tester).

## Requirements

Compile-time:

- clang,
- git,
- libelf,
- pkg-config,
- zlib.

### Nix (Linux / Mac)

``` bash
$ nix develop
```

### Ubuntu

```bash
$ sudo apt install clang libelf-dev pkg-config
```

### Mac

```bash
$ brew install libelf pkg-config
```

## Feature flags

All feature flags are disabled by default - if you want, you can activate:

- `patch-twi-inconsistencies` - applies a fix for https://github.com/buserror/simavr/issues/453

## FAQ

### How does it compare to `simavr-sys`?

[`simavr-sys`](https://github.com/dylanmckay/simavr-sim) seems unmaintained at
the moment, while `simavr-ffi` is up-to-date.

## License

Copyright (c) 2022 Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
