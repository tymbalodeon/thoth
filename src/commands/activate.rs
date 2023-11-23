use indoc::formatdoc;

pub fn activate_main() {
    println!(
        "{}",
        formatdoc!(
            "
_thoth_hook() {{
    echo THOTH
}}

typeset -ag precmd_functions;

if [[ -z \"${{precmd_functions[(r)_thoth_hook]+1}}\" ]]; then
  precmd_functions=( _thoth_hook ${{precmd_functions[@]}} )
fi

typeset -ag chpwd_functions;

if [[ -z \"${{chpwd_functions[(r)_thoth_hook]+1}}\" ]]; then
  chpwd_functions=( _thoth_hook ${{chpwd_functions[@]}} )
fi"
        )
    );
}
