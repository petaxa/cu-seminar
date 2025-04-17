# README

ゼミのためのあれこれを突っ込んでおくもの

## Settings

1. cargo-make のインストール

  ```bash
  cargo install --force cargo-make
  ```

## Commands

基本的に cargo-make を利用する。
`task-name`、実行時引数については `Makefile.toml` を確認してください。
`{@}` や `%*` が利用されているタスクは実行時引数を受け取れます。

```bash
makers <task-name> <arg1> <arg2> ...
```
