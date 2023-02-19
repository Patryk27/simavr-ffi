# simavr-ffi &emsp; [![crates-badge]][crates-link]

[crates-badge]: https://img.shields.io/crates/v/simavr-ffi.svg
[crates-link]: https://crates.io/crates/simavr-ffi

Low-level bindings to [simavr](https://github.com/buserror/simavr).

Status: alpha; work in progress.

Supported platforms: Linux & Mac (pull requests for Windows welcome!).

## Requirements

Compile-time:

- clang,
- libelf,
- pkg-config,
- zlib.

### Nix (Linux)

``` bash
$ nix-shell
```

### Ubuntu

```bash
$ sudo apt install clang libelf-dev pkg-config
```

### Mac

```bash
$ brew tap osx-cross/avr
$ brew install avr-gcc libelf pkg-config
```

## FAQ

### How does it compare to `simavr-sys`?

[`simavr-sys`](https://github.com/dylanmckay/simavr-sim) seems unmaintained at
the moment, so `simavr-ffi` is just more up-to-date.

## License

Copyright (c) 2022-2023, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
