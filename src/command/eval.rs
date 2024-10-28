use nu_plugin::PluginCommand;

use crate::custom;
use nu_plugin_helpers::{Command, FromValues};

#[derive(FromValues)]
struct Args<'a> {
    /// lua instance
    lua: &'a custom::Lua,
    /// lua code to evaluate
    code: &'a str,
}

pub fn eval() -> Box<dyn PluginCommand<Plugin = crate::Plugin>> {
    Command::<crate::Plugin>::new::<Args>(
        "lua eval",
        "evaluate lua to a nushell value",
        |plugin, _engine, Args { lua, code }| plugin.eval_lua(lua, code),
    )
    .into()
}
