# simavr-ffi

Low-level bindings to [simavr](https://github.com/buserror/simavr).

Status: work in progress, pretty alpha, not yet released.

## Requirements

- clang (with `LIBCLANG_PATH`),
- libelf,
- pkg-config.

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

### Why not `simavr-sys`?

[`simavr-sys`](https://github.com/dylanmckay/simavr-sim) seems unmaintained at
the moment.

## License

Copyright (c) 2022, Patryk Wychowaniec <pwychowaniec@pm.me>.    
Licensed under the MIT license.
