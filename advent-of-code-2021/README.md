# Advent of Code 2021

## Run

In order to have all thing working, put in files under `./src/testdata` your input files.

```zsh
zig run ./src/main.zig
```

If you want to check implementation with test

```zsh
zig ./src/day01.zig
```

If you don't have a rust environment you can create as follow with different tools.

### Devbox 

https://github.com/jetify-com/devbox

```zsh
curl -fsSL https://get.jetify.com/devbox | bash

cd <here>
devbox install
devbox shell
which zig # installed using devbox + nix
```

### Activate those with direnv

> `eval "$(devbox generate direnv --print-envrc)"`
