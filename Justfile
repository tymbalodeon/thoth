@_help:
    just --list

# Check code for issues, optionally using "clippy".
@check:
    cargo check

# Check code for issues using "clippy".
clippy:
    #!/usr/bin/env zsh
    cargo clippy -- \
        -W clippy::pedantic
        -W clippy::nursery \
        -W clippy::unwrap_used

# Run the application, with any provided <args>.
@try *args:
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
