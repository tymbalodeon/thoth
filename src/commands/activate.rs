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
        Shell::Nu => println!(
            "
def --env thoth-hook [] {{
  if (command -v thoth | complete | get exit_code | into bool) {{
    return
  }}

  let lilypond_path = (thoth update-path nu | split row \":\" | first)  

  if ($lilypond_path | is-empty) {{
    return
  }}

  $env.PATH = (
    $env.PATH 
    | split row \":\" 
    | filter {{ |path| not (\"lilypond\" in $path) }}
  )

  $env.PATH = ($env.PATH | prepend $lilypond_path)
}}

export-env {{
  $env.config = (
    $env.config 
    | upsert hooks {{
      pre_prompt: (
        $env.config.hooks.pre_prompt ++ [
          {{ || thoth-hook }}
        ]
      )
      env_change: {{
        PWD: (
          $env.config.hooks.env_change.PWD ++ [
            {{ || thoth-hook }}
          ]
        )
      }}
  }})
}}",
        ),
        Shell::Zsh => {
            println!("{}", formatdoc!(
                "
_thoth_update_path() {{
    export PATH=\"$(\"${{HOME}}\"/.cargo/bin/thoth update-path zsh)\"
}}

typeset -ag precmd_functions;

if [[ -z \"${{precmd_functions[(r)_thoth_update_path]+1}}\" ]]; then
  precmd_functions=( _thoth_update_path ${{precmd_functions[@]}} )
fi

typeset -ag chpwd_functions;

if [[ -z \"${{chpwd_functions[(r)_thoth_update_path]+1}}\" ]]; then
  chpwd_functions=( _thoth_update_path ${{chpwd_functions[@]}} )
fi"
            ));
        }
    }
}
