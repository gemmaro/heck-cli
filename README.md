# heck CLI

This is a program that simply turns the `heck` crate into a Command Line Interface (CLI).

```shell-session
$ echo 'This is a heck CLI' | heck
this-is-a-heck-cli

$ echo 'This is a heck CLI' | heck -t shoutysnake
THIS_IS_A_HECK_CLI

$ heck -h # to show usage
```

## Install

```shell-session
$ cargo install --git <GIT_URL>
```

Not registered in crates.io.

## License

`heck-cli` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
