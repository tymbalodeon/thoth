set shell := ["nu", "-c"]

_help:
    #!/usr/bin/env nu

    (
        just --list
            --color always
            --list-heading (
                [
                    "Available recipes:"
                    "(run `<recipe> --help/-h` for more info)\n"
                ]
                | str join " "
            )
    )

alias source := src

# Display the source code for a recipe
src recipe *args="_":
    #!/usr/bin/env nu

    # Display the source code for a recipe. If no args are provided, display
    # the raw `just` code, otherwise display the code with the args provided
    # to `just` applied. Pass `""` as args to see the code when no args are
    # provided to a recipe, and to see the code with `just` variables expanded.
    def src [
        recipe: string # The recipe command
        ...args: string # Arguments to the recipe
    ] {
        if "_" in $args {
            just --show $recipe
        } else {
            just --dry-run $recipe $args
        }
    }

    src {{ recipe }} `{{ args }}`

# Search available `just` commands
[no-exit-message]
find *regex:
    #!/usr/bin/env nu

    # Search available `just` commands interactively, or by <regex>
    def find [
        regex?: string # Regex pattern to match
    ] {
        if ($regex | is-empty) {
            just --list | fzf
        } else {
            just | grep --color=always --extended-regexp $regex
        }
    }

    find {{ regex }}

# Manage project Rust version
rust *args:
    #!/usr/bin/env nu

    # Manage project Rust version
    def rust [
        --installed # Show installed Rust versions
        --path # Show the path of the current Rust
        --use: string # Specify a new Rust version to use
        --version # (default) Show the current Rust version
    ] {
        if $installed {
            rustup toolchain list
            exit
        } else if $path {
            rustup which rustc
            exit
        } else if $version or ($use | is-empty) {
            rustc --version
            exit
        }

        let file = "rust-toolchain.toml"

        open $file
        | update toolchain.channel $use
        | save --force $file
    }

    rust {{ args }}

# Add dependencies
add *args:
    #!/usr/bin/env nu

    # Add dependencies
    def add [
        ...dependencies: string, # Dependencies to add
        --features: list<string> # Features to enable ("[<dependency>/<feature> ...]")
    ]: {
        let features = (
            $features
            | each { |feature| $"-F ($feature)" }
        )

        cargo add $dependencies $features
    }

    add {{ args }}

# Remove dependencies
remove *args:
    #!/usr/bin/env nu

    # Remove dependencies
    def remove [
        ...dependencies: string # Dependencies to remove
    ] {
        for dependency in $dependencies {
            cargo remove $dependency
        }
    }

    remove {{ args }}

# Update dependencies
update *args:
    #!/usr/bin/env nu

    # Update dependencies
    def update [] {
        cargo update
    }

    update {{ args }}

# Show application dependencies
dependencies *args:
    #!/usr/bin/env nu

    # Show application dependencies
    def dependencies [] {
        cargo tree --depth 1
    }

    dependencies {{ args }}

# Check code for issues
@check:
    cargo check

# Check code for issues using "clippy"
clippy:
    #!/usr/bin/env zsh
    cargo clippy -- \
        -W clippy::pedantic \
        -A clippy::too_many_lines \
        -A clippy::fn_params_excessive_bools  \
        -A clippy::module_name_repetitions \
        -A clippy::too_many_arguments \
        -W clippy::nursery \
        -W clippy::unwrap_used

# Run the application, with any provided <args>.
@run *args:
    cargo run -- {{ args }} 

# Build the application
build *args:
    #!/usr/bin/env nu

    # Build the application
    def build [
        --release # Build in release mode, with optimizations
    ] {
        if $release {
            cargo build --release
        } else {
            cargo build
        }
     }

    build {{ args }}

# Install the application
install *args:
    #!/usr/bin/env nu

    # Install the application
    def install [] {
        cargo install --path .
    }

    install {{ args }}
