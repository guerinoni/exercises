# Advent of Code 2024

## Run

In order to have all thing working, put in files under `./src/testdata` your input files.

```zsh
cargo run
```

If you want to check implementation with test

```zsh
cargo test
```

If you don't have a rust environment you can create as follow with different tools.

### Devbox 

https://github.com/jetify-com/devbox

```zsh
curl -fsSL https://get.jetify.com/devbox | bash

cd <here>
devbox install
devbox shell
which cargo # installed using devbox + nix
```

### Flox

https://github.com/flox/flox

```zsh
brew install --cask flox

cd <here>
flox activate
which cargo # installed using flox + nix
```

### Activate those with direnv

> `eval "$(devbox generate direnv --print-envrc)"`

or 

> `. <(flox activate)`