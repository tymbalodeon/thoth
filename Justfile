@_help:
    just --list

# Check code for issues, optionally using "clippy".
@check *clippy:
    cargo {{ if clippy == "--clippy" { "clippy" } else { "check" } }}

# Check code for issues using "clippy".
@clippy:
    just check --clippy
    # cargo clippy --fix -- -W clippy::pedantic/nursery/unwrap_used

# Run the application, with any provided <args>.
try *args:
    #!/usr/bin/env zsh
    cargo run -- {{args}} {{ if args == "" { "|| exit 0" } else { "" } }}

# Add a dependency.
@add +dependencies:
    cargo add {{dependencies}}

# Remove a dependency.
@remove +dependencies:
    cargo remove {{dependencies}}

# Install the application.
@install:
    cargo install --path .

# List the dependencies.
@list:
    cargo tree --depth 1

# Update the dependencies.
@update:
    cargo update
