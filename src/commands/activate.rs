use clap::ValueEnum;
use indoc::formatdoc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, ValueEnum)]
pub enum Shell {
    Nu,
    Zsh,
}

pub fn main(shell: &Shell) {
    match shell {
        Shell::Nu => println!(),
        Shell::Zsh => {
            println!(
                "{}",
                formatdoc!(
                    "
_thoth_update_path() {{
    eval PATH=\"$(\"${{HOME}}\"/.cargo/bin/thoth update-path)\"
}}

typeset -ag precmd_functions;

if [[ -z \"${{precmd_functions[(r)_thoth_update_path]+1}}\" ]]; then
  precmd_functions=( _thoth_update_path ${{precmd_functions[@]}} )
fi

typeset -ag chpwd_functions;

if [[ -z \"${{chpwd_functions[(r)_thoth_update_path]+1}}\" ]]; then
  chpwd_functions=( _thoth_update_path ${{chpwd_functions[@]}} )
fi"
                )
            );
        }
    }
}
