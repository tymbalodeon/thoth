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

@direct-asset-url:
    #!/usr/bin/env zsh
    url="$(curl \
            'https://gitlab.com/api/v4/projects/18695663/releases/v2.24.3' \
        | jq \
            '.assets.links | .[] | select(.name | contains("darwin")) | .direct_asset_url'
    )"
    printf "${url//\"/}"

@download:
    curl -L "$(just direct-asset-url)" -o ~/Desktop/lilypond.tar.gz
