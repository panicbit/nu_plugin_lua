set shell := ["nu", "-c"]

test:
    #!nu
    cargo build

    let executable = (
        cargo metadata --format-version 1
        | from json
        | get target_directory
        | path join debug nu_plugin_lua.exe 
    )
    let plugins = [$executable] | to nuon
    let set_prompt = '$env.PROMPT_COMMAND = $"(ansi red)TEST(ansi reset)>"'

    nu --plugins $plugins -e $set_prompt
