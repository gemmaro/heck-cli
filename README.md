# heck CLI

```shell-session
$ echo 'This is a heck CLI' | heck
this-is-a-heck-cli

$ echo 'This is a heck CLI' | heck -t shouty-snake
THIS_IS_A_HECK_CLI
```

## Install

```
$ cargo install --git https://vcs.gemmaro.mydns.jp/git/gemmaro/heck-cli.git
```

## Cases

Please refer to `TargetCase` enum variants in `src/bin/heck.rs`.

## License

`heck-cli` is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See `LICENSE-APACHE` and `LICENSE-MIT` for details.
