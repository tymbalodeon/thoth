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
    # provided to a recipe.
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

# Check code for issues, optionally using "clippy".
@check:
    cargo check

# Check code for issues using "clippy".
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

# Add a dependency.
@add +dependencies:
    cargo add {{dependencies}}

# Remove a dependency.
@remove +dependencies:
    cargo remove {{dependencies}}

# Install the application.
@install:
    cargo install --path .

# Update the dependencies.
@update:
    cargo update

# List the dependencies.
@dependencies:
    cargo tree --depth 1

