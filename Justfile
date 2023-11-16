@_help:
    just --list

# Check code for issues, optionally using "clippy".
@check *clippy:
    cargo {{ if clippy == "--clippy" { "clippy" } else { "check" } }}

# Check code for issues using "clippy".
@clippy:
    just check --clippy

# Run the application, with any provided <args>.
try *args:
    #!/usr/bin/env zsh
    cargo run -- {{args}} {{ if args == "" { "|| exit 0" } else { "" } }}

# Install the application.
@install:
    cargo install --path .

# Update the dependencies.
@update:
    cargo update
