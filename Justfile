@_help:
    just --list

@check *clippy:
    cargo {{ if clippy == "--clippy" { "clippy" } else { "check" } }}

@clippy:
    just check --clippy

try *args:
    #!/usr/bin/env zsh
    cargo run -- {{args}} {{ if args == "" { "|| exit 0" } else { "" } }}

@install:
    cargo install --path .
