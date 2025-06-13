# simavr-ffi &emsp; [![crates-badge]][crates-link]

[crates-badge]: https://img.shields.io/crates/v/simavr-ffi.svg
[crates-link]: https://crates.io/crates/simavr-ffi

Low-level bindings to [simavr](https://github.com/buserror/simavr):

```toml
[dependencies]
simavr-ffi = "1"
```

Supported platforms: Linux & Mac (pull requests for other platforms welcome!).

If you're looking for something high-level, see [avr-tester](https://github.com/Patryk27/avr-tester).

## Requirements

Compile-time:

- clang,
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

## License

Copyright (c) 2022 Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
